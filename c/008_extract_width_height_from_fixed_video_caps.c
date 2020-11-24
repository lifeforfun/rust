#include<stdio.h>
#include<gst/gst.h>

static void link_to_multiplexer(GstPad *tolink_pad, GstElement *mux)
{
    GstPad *pad;
    gchar *srcname, *sinkname;
    srcname = gst_pad_get_name(tolink_pad);
    pad = gst_element_get_compatible_pad(mux, tolink_pad, NULL);
    gst_pad_link(tolink_pad, pad);
    sinkname = gst_pad_get_name(pad);
    gst_object_unref(GST_OBJECT(pad));
    g_print("A new pad %s was created and linked to %s\n", sinkname, srcname);
    g_free(sinkname);
    g_free(srcname);
}

static void cb_new_pad(GstElement *element, GstPad *pad, gpointer data)
{
    gchar *name;
    name = gst_pad_get_name(pad);
    g_print("A new pad %s was created\n", name);
    g_free(name);
    link_to_multiplexer(pad, element);
}

static void read_video_props(GstCaps *caps)
{
    gint width, height;
    const GstStructure *str;
    g_return_if_fail(gst_caps_is_fixed(caps));
    str = gst_caps_get_structure(caps, 0);
    if (!gst_structure_get_int(str, "width", &width) ||
        !gst_structure_get_int(str, "height", &height)) {
        g_print("No width/height available\n");
        return;
    }
    g_print("The video size of this set of capabilities is %dx%d\n",
            width, height);
}

int main(int argc, char *argv[])
{
    GstElement *pipeline, *source, *demux;
    GMainLoop *loop;
    gst_init(&argc, &argv);
    pipeline = gst_pipeline_new("my_pipeline");
    source = gst_element_factory_make("filesrc", "source");
    g_object_set(source, "location", argv[1], NULL);
    demux = gst_element_factory_make("oggdemux", "demuxer");
    gst_bin_add_many(GST_BIN(pipeline), source, demux, NULL);
    gst_element_link_pads(source, "src", demux, "sink");
    g_signal_connect(demux, "pad-added", G_CALLBACK(cb_new_pad), NULL);
    gst_element_set_state(GST_ELEMENT(pipeline), GST_STATE_PLAYING);
    loop = g_main_loop_new(NULL, FALSE);
    g_main_loop_run(loop);
    return 0;
}
