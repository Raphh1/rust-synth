using System.Threading.Tasks;
using Mase.Ui.Services;
using Mase.Ui.ViewModels;
using Moq;
using Xunit;

namespace Mase.Ui.Tests.ViewModels;

public class MainViewModelTests
{
    private static (MainViewModel vm, Mock<IIpcService> mock) Build(bool ipcReturns = true)
    {
        var mock = new Mock<IIpcService>();
        mock.Setup(s => s.SendCommandAsync(
                It.IsAny<string>(), It.IsAny<string>(), It.IsAny<object?>()))
            .ReturnsAsync(ipcReturns);
        var vm = new MainViewModel(mock.Object);
        return (vm, mock);
    }

    // ─── BPM ─────────────────────────────────────────────────────────────────

    [Fact]
    public void BpmText_AboveMax_ClampsTo250()
    {
        var (vm, _) = Build();
        vm.BpmText = "999";
        Assert.Equal(250, vm.Bpm);
    }

    [Fact]
    public void BpmText_BelowMin_ClampsTo40()
    {
        var (vm, _) = Build();
        vm.BpmText = "5";
        Assert.Equal(40, vm.Bpm);
    }

    [Fact]
    public void BpmText_InvalidString_DoesNotChangeBpm()
    {
        var (vm, _) = Build();
        var before = vm.Bpm;
        vm.BpmText = "abc";
        Assert.Equal(before, vm.Bpm);
    }

    [Fact]
    public void Bpm_SyncsBpmText()
    {
        var (vm, _) = Build();
        vm.Bpm = 150;
        Assert.Equal("150", vm.BpmText);
    }

    [Fact]
    public void BpmUp_IncrementsByOne()
    {
        var (vm, _) = Build();
        var before = vm.Bpm;
        vm.BpmUpCommand.Execute(null);
        Assert.Equal(before + 1, vm.Bpm);
    }

    [Fact]
    public void BpmDown_DecrementsByOne()
    {
        var (vm, _) = Build();
        vm.Bpm = 120;
        vm.BpmDownCommand.Execute(null);
        Assert.Equal(119, vm.Bpm);
    }

    [Fact]
    public void BpmUp_AtMax_DoesNotExceed250()
    {
        var (vm, _) = Build();
        vm.Bpm = 250;
        vm.BpmUpCommand.Execute(null);
        Assert.Equal(250, vm.Bpm);
    }

    // ─── Play / Stop ─────────────────────────────────────────────────────────

    [Fact]
    public async Task PlayAsync_OnSuccess_UpdatesState()
    {
        var (vm, _) = Build(ipcReturns: true);
        await vm.PlayCommand.ExecuteAsync(null);

        Assert.Equal("Playing ▶▶", vm.PlayButtonText);
        Assert.Equal("Playing",    vm.EngineStateText);
        Assert.Equal("#4CAF50",    vm.EngineStateColor);
    }

    [Fact]
    public async Task PlayAsync_OnFailure_DoesNotChangeState()
    {
        var (vm, _) = Build(ipcReturns: false);
        await vm.PlayCommand.ExecuteAsync(null);

        Assert.Equal("Play ▶", vm.PlayButtonText);
    }

    [Fact]
    public async Task PlayAsync_WhenAlreadyPlaying_DoesNotSendCommand()
    {
        var (vm, mock) = Build(ipcReturns: true);
        await vm.PlayCommand.ExecuteAsync(null);
        await vm.PlayCommand.ExecuteAsync(null);

        mock.Verify(s => s.SendCommandAsync("play", It.IsAny<string>(), It.IsAny<object?>()),
            Times.Once);
    }

    [Fact]
    public async Task StopAsync_OnSuccess_ResetsState()
    {
        var (vm, _) = Build(ipcReturns: true);
        await vm.PlayCommand.ExecuteAsync(null);
        await vm.StopCommand.ExecuteAsync(null);

        Assert.Equal("Play ▶",    vm.PlayButtonText);
        Assert.Equal("Stopped",   vm.EngineStateText);
        Assert.Equal("#de0013ff", vm.EngineStateColor);
    }

    [Fact]
    public async Task StopAsync_WhenNotPlaying_DoesNotSendCommand()
    {
        var (vm, mock) = Build(ipcReturns: true);
        await vm.StopCommand.ExecuteAsync(null);

        mock.Verify(s => s.SendCommandAsync("stop", It.IsAny<string>(), It.IsAny<object?>()),
            Times.Never);
    }

    // ─── Loop ────────────────────────────────────────────────────────────────

    [Fact]
    public async Task ToggleLoop_EnablesLoop()
    {
        var (vm, _) = Build(ipcReturns: true);
        await vm.ToggleLoopCommand.ExecuteAsync(null);

        Assert.Equal("Loop ↩ ON", vm.LoopButtonText);
        Assert.Equal("#FF8F00",   vm.LoopButtonColor);
    }

    [Fact]
    public async Task ToggleLoop_Twice_DisablesLoop()
    {
        var (vm, _) = Build(ipcReturns: true);
        await vm.ToggleLoopCommand.ExecuteAsync(null);
        await vm.ToggleLoopCommand.ExecuteAsync(null);

        Assert.Equal("Loop ↩",  vm.LoopButtonText);
        Assert.Equal("#3E3E42", vm.LoopButtonColor);
    }

    [Fact]
    public async Task ToggleLoop_OnFailure_RevertsState()
    {
        var (vm, _) = Build(ipcReturns: false);
        await vm.ToggleLoopCommand.ExecuteAsync(null);

        Assert.Equal("Loop ↩", vm.LoopButtonText);
    }

    // ─── Portamento display ───────────────────────────────────────────────────

    [Fact]
    public void Portamento_Zero_DisplaysOff()
    {
        var (vm, _) = Build();
        vm.Portamento = 0.0;
        Assert.Equal("Off", vm.PortamentoDisplay);
    }

    [Fact]
    public void Portamento_NonZero_DisplaysTime()
    {
        var (vm, _) = Build();
        vm.Portamento = 0.5;
        Assert.Equal("0.50s", vm.PortamentoDisplay);
    }

    // ─── Filter display ───────────────────────────────────────────────────────

    [Fact]
    public void Cutoff_UpdatesDisplay()
    {
        var (vm, _) = Build();
        vm.Cutoff = 5000;
        Assert.Equal("Cutoff: 5000 Hz", vm.CutoffDisplay);
    }

    [Fact]
    public void Resonance_UpdatesDisplay()
    {
        var (vm, _) = Build();
        vm.Resonance = 0.75;
        Assert.Equal("Resonance: 0.75", vm.ResonanceDisplay);
    }

    // ─── LFO shape ────────────────────────────────────────────────────────────

    [Fact]
    public async Task SetLfoShape_Square_HighlightsSquareButton()
    {
        var (vm, _) = Build();
        await vm.SetLfoShapeCommand.ExecuteAsync("square");

        Assert.Equal("#1565C0", vm.LfoSquareColor);
        Assert.Equal("#2D2D30", vm.LfoSineColor);
        Assert.Equal("#2D2D30", vm.LfoTriColor);
        Assert.Equal("#2D2D30", vm.LfoSawColor);
    }

    [Fact]
    public async Task SetLfoShape_Sine_HighlightsSineButton()
    {
        var (vm, _) = Build();
        await vm.SetLfoShapeCommand.ExecuteAsync("square");
        await vm.SetLfoShapeCommand.ExecuteAsync("sine");

        Assert.Equal("#1565C0", vm.LfoSineColor);
        Assert.Equal("#2D2D30", vm.LfoSquareColor);
    }

    // ─── LFO target ───────────────────────────────────────────────────────────

    [Fact]
    public async Task SetLfoTarget_Pitch_HighlightsPitchButton()
    {
        var (vm, _) = Build();
        await vm.SetLfoTargetCommand.ExecuteAsync("pitch");

        Assert.Equal("#1565C0", vm.LfoPitchColor);
        Assert.Equal("#2D2D30", vm.LfoCutoffColor);
        Assert.Equal("#2D2D30", vm.LfoVolumeColor);
    }

    [Fact]
    public async Task SetLfoTarget_Volume_HighlightsVolumeButton()
    {
        var (vm, _) = Build();
        await vm.SetLfoTargetCommand.ExecuteAsync("volume");

        Assert.Equal("#1565C0", vm.LfoVolumeColor);
        Assert.Equal("#2D2D30", vm.LfoPitchColor);
    }

    // ─── Waveform edit mode ───────────────────────────────────────────────────

    [Fact]
    public void ToggleWaveformEdit_CanExecute_WhenNotPlaying()
    {
        var (vm, _) = Build();
        Assert.True(vm.ToggleWaveformEditCommand.CanExecute(null));
    }

    [Fact]
    public async Task ToggleWaveformEdit_CannotExecute_WhenPlaying()
    {
        var (vm, _) = Build(ipcReturns: true);
        await vm.PlayCommand.ExecuteAsync(null);
        Assert.False(vm.ToggleWaveformEditCommand.CanExecute(null));
    }

    [Fact]
    public void ToggleWaveformEdit_SwitchesToEditMode()
    {
        var (vm, _) = Build();
        vm.ToggleWaveformEditCommand.Execute(null);
        Assert.Equal("View",    vm.WaveformModeText);
        Assert.Equal("#1565C0", vm.WaveformModeColor);
    }

    // ─── Notes collection ────────────────────────────────────────────────────

    [Fact]
    public async Task AddNote_OnSuccess_AddsToCollection()
    {
        var (vm, _) = Build(ipcReturns: true);
        await vm.AddNoteAsync(60, 0);
        Assert.Single(vm.Notes);
        Assert.Equal(60, vm.Notes[0].Pitch);
    }

    [Fact]
    public async Task AddNote_OnFailure_DoesNotAddToCollection()
    {
        var (vm, _) = Build(ipcReturns: false);
        await vm.AddNoteAsync(60, 0);
        Assert.Empty(vm.Notes);
    }

    [Fact]
    public async Task DeleteNote_OnSuccess_RemovesFromCollection()
    {
        var (vm, _) = Build(ipcReturns: true);
        await vm.AddNoteAsync(60, 0);
        var id = vm.Notes[0].Id;

        await vm.DeleteNoteAsync(id);

        Assert.Empty(vm.Notes);
    }

    [Fact]
    public async Task DeleteNote_UnknownId_DoesNothing()
    {
        var (vm, _) = Build(ipcReturns: true);
        await vm.AddNoteAsync(60, 0);

        await vm.DeleteNoteAsync("nonexistent-id");

        Assert.Single(vm.Notes);
    }
}
