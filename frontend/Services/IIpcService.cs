using System;
using System.Threading.Tasks;

namespace Mase.Ui.Services;

public interface IIpcService : IDisposable
{
    void Connect();
    Task<bool> SendCommandAsync(string commandType, string requestId, object? payload = null);
}
