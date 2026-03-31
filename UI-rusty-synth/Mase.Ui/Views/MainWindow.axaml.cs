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

        WaveformCtrl.TableChanged     += OnTableChanged;
        EnvelopeCtrl.EnvelopeChanged  += OnEnvelopeChanged;
        DataContextChanged            += OnDataContextChanged;
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

        // Sync envelope control with initial values
        EnvelopeCtrl.A = vm.Attack;
        EnvelopeCtrl.D = vm.Decay;
        EnvelopeCtrl.S = vm.Sustain;
        EnvelopeCtrl.R = vm.Release;

        vm.PropertyChanged += (_, args) =>
        {
            if (_envelopeSyncing) return; // avoid feedback loop
            switch (args.PropertyName)
            {
                case nameof(vm.Attack):  EnvelopeCtrl.A = vm.Attack;  break;
                case nameof(vm.Decay):   EnvelopeCtrl.D = vm.Decay;   break;
                case nameof(vm.Sustain): EnvelopeCtrl.S = vm.Sustain; break;
                case nameof(vm.Release): EnvelopeCtrl.R = vm.Release; break;
            }
        };
    }

    private bool _envelopeSyncing = false;

    private void OnEnvelopeChanged(double a, double d, double s, double r)
    {
        if (DataContext is not MainViewModel vm) return;
        _envelopeSyncing = true;
        vm.Attack  = a;
        vm.Decay   = d;
        vm.Sustain = s;
        vm.Release = r;
        _envelopeSyncing = false;
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
