using Avalonia.Controls;
using Mase.Ui.Controls;
using Mase.Ui.ViewModels;

namespace Mase.Ui.Views;

public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();

        MainPianoRoll.NoteAdded   += OnNoteAdded;
        MainPianoRoll.NoteDeleted += OnNoteDeleted;
        MainPianoRoll.NoteMoved   += OnNoteMoved;
        MainPianoRoll.NoteResized += OnNoteResized;

        WaveformCtrl.TableChanged += OnTableChanged;
        DataContextChanged        += OnDataContextChanged;
    }

    // ─── Waveform ─────────────────────────────────────────────────────────────

    private void OnDataContextChanged(object? sender, System.EventArgs e)
    {
        if (DataContext is not MainViewModel vm) return;

        WaveformCtrl.Table = vm.WaveformTable;
        WaveformCtrl.InvalidateVisual();

        vm.EditModeChanged += isEdit =>
        {
            WaveformCtrl.Table = vm.WaveformTable;
            WaveformCtrl.Mode  = isEdit ? WaveformMode.Edit : WaveformMode.View;
            WaveformCtrl.InvalidateVisual();
        };
    }

    private async void OnTableChanged(double[] table)
    {
        if (DataContext is not MainViewModel vm) return;
        await vm.WavetableSetAsync(table);
    }

    // ─── Piano Roll ───────────────────────────────────────────────────────────

    private async void OnNoteAdded(object? sender, NoteAddedEventArgs e)
    {
        if (DataContext is not MainViewModel vm) return;
        await vm.AddNoteAsync(e.Pitch, e.Beat);
        MainPianoRoll.Notes = vm.Notes;
        MainPianoRoll.InvalidateVisual();
    }

    private async void OnNoteDeleted(object? sender, NoteDeletedEventArgs e)
    {
        if (DataContext is not MainViewModel vm) return;
        await vm.DeleteNoteAsync(e.NoteId);
        MainPianoRoll.Notes = vm.Notes;
        MainPianoRoll.InvalidateVisual();
    }

    private async void OnNoteMoved(object? sender, NoteMovedEventArgs e)
    {
        if (DataContext is not MainViewModel vm) return;
        await vm.MoveNoteAsync(e.NoteId, e.NewPitch, e.NewBeat);
        MainPianoRoll.InvalidateVisual();
    }

    private async void OnNoteResized(object? sender, NoteResizedEventArgs e)
    {
        if (DataContext is not MainViewModel vm) return;
        await vm.ResizeNoteAsync(e.NoteId, e.NewLength);
        MainPianoRoll.InvalidateVisual();
    }
}
