using System.IO;
using System.IO.Compression;
using EmbedIO;
using EmbedIO.Routing;
using EmbedIO.WebApi;
using Newtonsoft.Json;
using SixLabors.ImageSharp;
using SixLabors.ImageSharp.Formats.Png;

namespace Auracite;

public class EndStep : IStep
{
    public event IStep.CompletedDelegate? Completed;
    
    public void Run()
    {
        StartWebServer();
    }

    public void End()
    {
        Completed?.Invoke();
    }

    public string StepName()
    {
        return "Save Archive";
    }

    public string StepDescription()
    {
        return "Save your archived character ZIP.";
    }
    
    private class Controller : WebApiController
    {
        private EndStep _endStep;

        public Controller(EndStep endStep)
        {
            _endStep = endStep;
        }
        
        [Route(HttpVerbs.Get, "/download")]
        public void GetPackage()
        {
            Response.Headers.Set(HttpHeaderNames.AccessControlAllowOrigin,  "*");
            Response.ContentType = "application/zip";
            using var writer = HttpContext.OpenResponseStream(true);

            using (var archive = new ZipArchive(writer, ZipArchiveMode.Create, true))
            {
                using (var entryStream = archive.CreateEntry("character.json").Open())
                {
                    using (var streamWriter = new StreamWriter(entryStream))
                    {
                        streamWriter.Write(JsonConvert.SerializeObject(Plugin.package, Formatting.Indented));
                    }
                }

                WriteImage(archive, Plugin.accent, "accent.png");
                WriteImage(archive, Plugin.backing, "backing.png");
                WriteImage(archive, Plugin.base_plate, "base-plate.png");
                WriteImage(archive, Plugin.pattern_overlay, "pattern-overlay.png");
                WriteImage(archive, Plugin.plate_frame, "plate-frame.png");
                WriteImage(archive, Plugin.portrait, "plate-portrait.png");
                WriteImage(archive, Plugin.top_border, "top-border.png");
                WriteImage(archive, Plugin.bottom_border, "bottom-border.png");
            }
        }
    }
    
    private WebServer? _server;
    
    private void StartWebServer()
    {
        ShutdownWebServer();

        _server = new WebServer(o => o
                .WithUrlPrefix("http://localhost:42072/")
                .WithMode(HttpListenerMode.EmbedIO))
            .WithWebApi("/", m => m.WithController(() => new Controller(this)));
        _server.RunAsync();
    }

    private void ShutdownWebServer()
    {
        _server?.Dispose();
        _server = null;
    }

    public void Dispose()
    {
        ShutdownWebServer();
    }

    public bool IsEnd()
    {
        return true;
    }

    private static void WriteImage(ZipArchive archive, Image? image, string path)
    {
        if (image != null)
        {
            using (var entryStream = archive.CreateEntry(path).Open())
            {
                image.Save(entryStream, PngFormat.Instance);
            }
        }
    }
}
