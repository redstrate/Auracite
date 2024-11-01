using System;
using System.Text;
using System.Threading.Tasks;
using EmbedIO;
using EmbedIO.Routing;
using EmbedIO.WebApi;
using Newtonsoft.Json;

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
        return "Waiting for Upload";
    }

    public string StepDescription()
    {
        return "Run Auracite to archive your character.";
    }
    
    private class Controller : WebApiController
    {
        private EndStep _endStep;

        public Controller(EndStep endStep)
        {
            _endStep = endStep;
        }
        
        [Route(HttpVerbs.Get, "/package")]
        public void GetPackage()
        {
            Response.ContentType = MimeType.Json;
            using var writer = HttpContext.OpenResponseText(Encoding.UTF8, true);
            writer.Write(JsonConvert.SerializeObject(Plugin.package));
            
            _endStep.End();
        }
    }
    
    private WebServer? _server;
    
    private void StartWebServer()
    {
        ShutdownWebServer();

        _server = new WebServer(o => o
                .WithUrlPrefix("http://localhost:42072/")
                .WithMode(HttpListenerMode.EmbedIO))
            .WithCors("http://localhost:42072/")
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
}