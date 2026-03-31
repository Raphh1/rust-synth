using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Media;
using Mase.Ui.Models;
using System;
using System.Collections.Generic;
using System.Globalization;

namespace Mase.Ui.Controls;

public class NoteAddedEventArgs : EventArgs
{
    public int Pitch { get; init; }
    public int Beat  { get; init; }
}

public class NoteDeletedEventArgs : EventArgs
{
    public string NoteId { get; init; } = "";
}

public class NoteMovedEventArgs : EventArgs
{
    public string NoteId   { get; init; } = "";
    public int    NewPitch { get; init; }
    public int    NewBeat  { get; init; }
}

public class PianoRollGrid : Control
{
    public int BaseNote  { get; set; } = 36;
    public int NoteCount { get; set; } = 48;

    public List<NoteModel> Notes { get; set; } = new();

    public event EventHandler<NoteAddedEventArgs>?   NoteAdded;
    public event EventHandler<NoteDeletedEventArgs>? NoteDeleted;
    public event EventHandler<NoteMovedEventArgs>?   NoteMoved;

    private const int    BeatCount   = 16;
    private const int    BeatsPerBar = 4;
    private const double LabelWidth  = 36;

    private static readonly bool[] IsBlackKey =
        { false, true, false, true, false, false, true, false, true, false, true, false };

    private NoteModel? _dragging;
    private int        _dragOriginalPitch;
    private int        _dragOriginalBeat;
    private Point      _dragStartPos;
    private bool       _rightHeld;

    private NoteModel? HitTest(Point pos)
    {
        double rowH  = Bounds.Height / NoteCount;
        double gridW = Bounds.Width - LabelWidth;
        double beatW = gridW / BeatCount;

        foreach (var note in Notes)
        {
            int    row = NoteCount - 1 - (note.Pitch - BaseNote);
            double nx  = LabelWidth + note.Start * beatW;
            double ny  = row * rowH;
            double nw  = note.Length * beatW;

            if (pos.X >= nx && pos.X < nx + nw && pos.Y >= ny && pos.Y < ny + rowH)
                return note;
        }
        return null;
    }

    private (int pitch, int beat) PosToPitchBeat(Point pos)
    {
        double rowH  = Bounds.Height / NoteCount;
        double gridW = Bounds.Width - LabelWidth;
        double beatW = gridW / BeatCount;

        int row   = Math.Clamp((int)(pos.Y / rowH),                0, NoteCount - 1);
        int beat  = Math.Clamp((int)((pos.X - LabelWidth) / beatW), 0, BeatCount - 1);
        return (BaseNote + (NoteCount - 1 - row), beat);
    }

    protected override void OnPointerPressed(PointerPressedEventArgs e)
    {
        base.OnPointerPressed(e);
        var pos   = e.GetPosition(this);
        var props = e.GetCurrentPoint(this).Properties;

        if (pos.X <= LabelWidth) return;

        var hit = HitTest(pos);

        if (props.IsRightButtonPressed)
        {
            _rightHeld = true;
            e.Pointer.Capture(this);
            if (hit != null)
                NoteDeleted?.Invoke(this, new NoteDeletedEventArgs { NoteId = hit.Id });
            return;
        }

        if (props.IsLeftButtonPressed)
        {
            if (hit != null)
            {
                _dragging          = hit;
                _dragOriginalPitch = hit.Pitch;
                _dragOriginalBeat  = hit.Start;
                _dragStartPos      = pos;
                e.Pointer.Capture(this);
            }
            else
            {
                var (pitch, beat) = PosToPitchBeat(pos);
                NoteAdded?.Invoke(this, new NoteAddedEventArgs { Pitch = pitch, Beat = beat });
            }
        }
    }

    protected override void OnPointerMoved(PointerEventArgs e)
    {
        base.OnPointerMoved(e);

        if (_rightHeld)
        {
            var hit = HitTest(e.GetPosition(this));
            if (hit != null)
                NoteDeleted?.Invoke(this, new NoteDeletedEventArgs { NoteId = hit.Id });
            return;
        }

        if (_dragging == null) return;

        var    pos   = e.GetPosition(this);
        double rowH  = Bounds.Height / NoteCount;
        double beatW = (Bounds.Width - LabelWidth) / BeatCount;

        int deltaBeat  = (int)Math.Round((pos.X - _dragStartPos.X) / beatW);
        int deltaPitch = -(int)Math.Round((pos.Y - _dragStartPos.Y) / rowH);

        _dragging.Pitch = Math.Clamp(_dragOriginalPitch + deltaPitch, BaseNote, BaseNote + NoteCount - 1);
        _dragging.Start = Math.Clamp(_dragOriginalBeat  + deltaBeat,  0,        BeatCount - 1);

        InvalidateVisual();
    }

    protected override void OnPointerReleased(PointerReleasedEventArgs e)
    {
        base.OnPointerReleased(e);

        if (_rightHeld)
        {
            _rightHeld = false;
            e.Pointer.Capture(null);
            return;
        }

        if (_dragging == null) return;

        var moved = _dragging;
        _dragging = null;
        e.Pointer.Capture(null);

        if (moved.Pitch != _dragOriginalPitch || moved.Start != _dragOriginalBeat)
            NoteMoved?.Invoke(this, new NoteMovedEventArgs { NoteId = moved.Id, NewPitch = moved.Pitch, NewBeat = moved.Start });

        InvalidateVisual();
    }

    public override void Render(DrawingContext ctx)
    {
        double w = Bounds.Width;
        double h = Bounds.Height;
        if (w <= 0 || h <= 0) return;

        double rowH     = h / NoteCount;
        double octaveH  = rowH * 12;
        double fontSize = Math.Clamp(octaveH * 0.25, 9, 16);
        double gridW    = w - LabelWidth;
        double beatW    = gridW / BeatCount;

        ctx.FillRectangle(new SolidColorBrush(Color.Parse("#141414")), new Rect(0, 0, w, h));

        for (int i = 0; i < NoteCount; i++)
        {
            int    midi     = BaseNote + (NoteCount - 1 - i);
            int    semitone = midi % 12;
            double y        = i * rowH;

            ctx.FillRectangle(
                IsBlackKey[semitone]
                    ? new SolidColorBrush(Color.Parse("#0F0F0F"))
                    : new SolidColorBrush(Color.Parse("#1C1C1C")),
                new Rect(LabelWidth, y, gridW, rowH));

            if (semitone == 0)
                ctx.DrawLine(new Pen(new SolidColorBrush(Color.Parse("#505050")), 1),
                    new Point(LabelWidth, y), new Point(w, y));
            else
                ctx.DrawLine(new Pen(new SolidColorBrush(Color.Parse("#272727")), 0.5),
                    new Point(LabelWidth, y + rowH), new Point(w, y + rowH));
        }

        for (int b = 0; b <= BeatCount; b++)
        {
            double x = LabelWidth + b * beatW;
            ctx.DrawLine(
                b % BeatsPerBar == 0
                    ? new Pen(new SolidColorBrush(Color.Parse("#585858")), 1)
                    : new Pen(new SolidColorBrush(Color.Parse("#2A2A2A")), 0.5),
                new Point(x, 0), new Point(x, h));
        }

        foreach (var note in Notes)
        {
            int row = NoteCount - 1 - (note.Pitch - BaseNote);
            if (row < 0 || row >= NoteCount) continue;

            double nx = LabelWidth + note.Start * beatW;
            double ny = row * rowH;
            double nw = Math.Max(note.Length * beatW - 1, 2);
            double nh = rowH - 1;

            bool isDragging = note == _dragging;
            ctx.FillRectangle(
                new SolidColorBrush(isDragging ? Color.Parse("#81C784") : Color.Parse("#4CAF50")),
                new Rect(nx, ny, nw, nh));
            ctx.DrawRectangle(
                new Pen(new SolidColorBrush(Color.Parse("#A5D6A7")), 1),
                new Rect(nx, ny, nw, nh));
        }

        ctx.FillRectangle(new SolidColorBrush(Color.Parse("#1A1A1A")), new Rect(0, 0, LabelWidth, h));
        ctx.DrawLine(new Pen(new SolidColorBrush(Color.Parse("#3E3E42")), 1),
            new Point(LabelWidth, 0), new Point(LabelWidth, h));

        var typeface = new Typeface("Inter");
        for (int i = 0; i < NoteCount; i++)
        {
            int midi     = BaseNote + (NoteCount - 1 - i);
            int semitone = midi % 12;
            if (semitone != 0) continue;

            int    octave = midi / 12 - 1;
            double y      = i * rowH;
            var    ft     = new FormattedText($"C{octave}", CultureInfo.InvariantCulture,
                                FlowDirection.LeftToRight, typeface, fontSize,
                                new SolidColorBrush(Color.Parse("#BBBBBB")));

            ctx.DrawText(ft, new Point(4, Math.Clamp(y + rowH / 2 - ft.Height / 2, 0, h - ft.Height)));
        }
    }
}
