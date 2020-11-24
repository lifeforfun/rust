#include<stdio.h>
#include<gst/gst.h>

static gboolean bus_call (GstBus *bus, GstMessage *msg, gpointer data)
{
    GMainLoop *loop = (GMainLoop *) data;
    switch(GST_MESSAGE_TYPE(msg)) {
        case GST_MESSAGE_EOS:
            g_print("End of stream\n");
            g_main_loop_quit(loop);
            break;

        case GST_MESSAGE_ERROR:
            {
                gchar *debug;
                GError *error;
                gst_message_parse_error(msg, &error, &debug);
                g_free(debug);
                g_printerr("Error: %s\n", error->message);
                g_error_free(error);
                g_main_loop_quit(loop);
                break;
            }
        default:
            break;
    }

    return TRUE;
}

static gboolean cb_print_position(GstElement *pipeline)
{
    gint64 pos, len;
    if (gst_element_query_position(pipeline, GST_FORMAT_TIME, &pos)
            && gst_element_query_duration(pipeline, GST_FORMAT_TIME, &len)) {
        g_print("Time: %" GST_TIME_FORMAT " / %" GST_TIME_FORMAT "\r",
                GST_TIME_ARGS(pos), GST_TIME_ARGS(len));
    }
    return TRUE;
}

static void on_pad_added(GstElement *element, GstPad *pad, gpointer data)
{
    GstPad *sinkpad;
    GstElement *decoder = (GstElement *) data;
    g_print("Dynamic pad created, linking demuxer/decoder\n");
    sinkpad = gst_element_get_static_pad(decoder, "sink");
    gst_pad_link(pad, sinkpad);
    gst_object_unref(sinkpad);
}

int main(int argc, char *argv[])
{
    GMainLoop *loop;

    gst_init(&argc, &argv);
    loop = g_main_loop_new(NULL, FALSE);
    if (argc!=2) {
        g_printerr("Usage: %s <Ogg/Vorbis filename>\n", argv[0]);
        return -1;
    }

    GstBus *bus;
    guint bus_watch_id;
    GstElement *pipeline, *source, *demuxer, *decoder, *sink;
    pipeline = gst_pipeline_new("audio-player");
    source = gst_element_factory_make("filesrc", NULL);
    demuxer = gst_element_factory_make("oggdemux", NULL);
    decoder = gst_element_factory_make("vorbisdec", NULL);
    sink = gst_element_factory_make("autoaudiosink", NULL);
    if (!pipeline || 
        !source ||
        !demuxer ||
        !decoder ||
        !sink ) {
        g_printerr("One element could not be created. Exiting. \n");
        return -1;
    }

    bus = gst_pipeline_get_bus(GST_PIPELINE(pipeline));
    bus_watch_id = gst_bus_add_watch(bus, bus_call, loop);
    gst_object_unref(bus);
    g_object_set(G_OBJECT(source), "location", argv[1], NULL);
    gst_bin_add_many(GST_BIN(pipeline),
            source, demuxer, decoder, sink, NULL);
    gst_element_link(source, demuxer);
    gst_element_link_many(decoder,sink,NULL);
    g_signal_connect(demuxer, "pad-added", G_CALLBACK(on_pad_added), decoder);
    g_print("Now playing: %s\n", argv[1]);
    gst_element_set_state(pipeline, GST_STATE_PLAYING);
    g_print("Running...\n");


    g_timeout_add(200, (GSourceFunc) cb_print_position, pipeline);
    g_main_loop_run(loop);
    g_print("Returned, stopping playback\n");
    gst_element_set_state(pipeline, GST_STATE_NULL);
    g_print("Deleting pipeline\n");
    gst_object_unref(GST_OBJECT(pipeline));
    g_main_loop_unref(loop);
    g_source_remove(bus_watch_id);
    return 0;
}
