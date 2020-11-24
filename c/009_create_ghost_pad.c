#include<stdio.h>
#include<gst/gst.h>

int main(int argc, char *argv[])
{
    GstElement *bin, *sink;
    GstPad *pad, *ghost_pad;
    gchar *bin_pad_name, *sink_pad_name;

    gst_init(&argc, &argv);
    sink = gst_element_factory_make("fakesink", "sink");
    bin = gst_bin_new("mybin");
    gst_bin_add(GST_BIN(bin), sink);
    pad = gst_element_get_static_pad(sink, "sink");
    gst_element_add_pad(bin, gst_ghost_pad_new("sink", pad));
    ghost_pad = gst_element_get_static_pad(bin, "sink");
    bin_pad_name = gst_pad_get_name(ghost_pad);
    sink_pad_name = gst_pad_get_name(pad);
    g_print("bin pad name is %s, sink pad name is %s.\n", bin_pad_name, sink_pad_name);
    gst_object_unref(GST_OBJECT(pad));
    gst_object_unref(GST_OBJECT(ghost_pad));
    g_free(bin_pad_name);
    g_free(sink_pad_name);
    return 0;
}
