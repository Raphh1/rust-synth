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

public class NoteResizedEventArgs : EventArgs
{
    public string NoteId    { get; init; } = "";
    public double NewLength { get; init; }
}

public class PianoRollGrid : Control
{
    public int BaseNote  { get; set; } = 36;
    public int NoteCount { get; set; } = 48;

    public List<NoteModel> Notes { get; set; } = new();

    public event EventHandler<NoteAddedEventArgs>?   NoteAdded;
    public event EventHandler<NoteDeletedEventArgs>? NoteDeleted;
    public event EventHandler<NoteMovedEventArgs>?   NoteMoved;
    public event EventHandler<NoteResizedEventArgs>? NoteResized;

    private const int    BeatCount         = 16;
    private const int    BeatsPerBar       = 4;
    private const double LabelWidth        = 36;
    private const double ResizeHandleWidth = 6;

    private static readonly bool[] IsBlackKey =
        { false, true, false, true, false, false, true, false, true, false, true, false };

    // Drag state
    private NoteModel? _dragging;
    private int        _dragOriginalPitch;
    private int        _dragOriginalBeat;
    private Point      _dragStartPos;

    // Resize state
    private NoteModel? _resizing;
    private double     _resizeOriginalLength;
    private double     _resizeStartX;

    // Erase state
    private bool _rightHeld;

    // ─── Hit tests ───────────────────────────────────────────────────────────

    private double BeatWidth => (Bounds.Width - LabelWidth) / BeatCount;
    private double RowHeight => Bounds.Height / NoteCount;

    // Retourne la note si le curseur est dans sa moitié droite
    private NoteModel? ResizeHitTest(Point pos)
    {
        double rowH  = RowHeight;
        double beatW = BeatWidth;

        foreach (var note in Notes)
        {
            int    row  = NoteCount - 1 - (note.Pitch - BaseNote);
            double nx   = LabelWidth + note.Start * beatW;
            double ny   = row * rowH;
            double nw   = note.Length * beatW;
            double mid  = nx + nw / 2.0;
            double right = nx + nw;

            if (pos.X >= mid && pos.X < right && pos.Y >= ny && pos.Y < ny + rowH)
                return note;
        }
        return null;
    }

    private NoteModel? HitTest(Point pos)
    {
        double rowH  = RowHeight;
        double beatW = BeatWidth;

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
        int row  = Math.Clamp((int)(pos.Y / RowHeight),                0, NoteCount - 1);
        int beat = Math.Clamp((int)((pos.X - LabelWidth) / BeatWidth), 0, BeatCount - 1);
        return (BaseNote + (NoteCount - 1 - row), beat);
    }

    // ─── Input ───────────────────────────────────────────────────────────────

    protected override void OnPointerPressed(PointerPressedEventArgs e)
    {
        base.OnPointerPressed(e);
        var pos   = e.GetPosition(this);
        var props = e.GetCurrentPoint(this).Properties;

        if (pos.X <= LabelWidth) return;

        if (props.IsRightButtonPressed)
        {
            _rightHeld = true;
            e.Pointer.Capture(this);
            var hit = HitTest(pos);
            if (hit != null)
                NoteDeleted?.Invoke(this, new NoteDeletedEventArgs { NoteId = hit.Id });
            return;
        }

        if (props.IsLeftButtonPressed)
        {
            // Resize a la priorité sur le drag
            var resizeHit = ResizeHitTest(pos);
            if (resizeHit != null)
            {
                _resizing             = resizeHit;
                _resizeOriginalLength = resizeHit.Length;
                _resizeStartX         = pos.X;
                e.Pointer.Capture(this);
                return;
            }

            var hit = HitTest(pos);
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
        var pos = e.GetPosition(this);

        if (_rightHeld)
        {
            var hit = HitTest(pos);
            if (hit != null)
                NoteDeleted?.Invoke(this, new NoteDeletedEventArgs { NoteId = hit.Id });
            return;
        }

        if (_resizing != null)
        {
            Cursor = new Cursor(StandardCursorType.SizeWestEast);
            const double snap = 0.25; // 16ème note
            double beatW   = BeatWidth;
            double nx      = LabelWidth + _resizing.Start * beatW;
            double raw     = (pos.X - nx) / beatW;
            double snapped = Math.Round(raw / snap) * snap;
            double newLen  = Math.Max(snap, Math.Min(snapped, BeatCount - _resizing.Start));
            _resizing.Length = newLen;
            InvalidateVisual();
            return;
        }

        if (_dragging != null)
        {
            double rowH       = RowHeight;
            double beatW      = BeatWidth;
            int    deltaBeat  = (int)Math.Round((pos.X - _dragStartPos.X) / beatW);
            int    deltaPitch = -(int)Math.Round((pos.Y - _dragStartPos.Y) / rowH);

            _dragging.Pitch = Math.Clamp(_dragOriginalPitch + deltaPitch, BaseNote, BaseNote + NoteCount - 1);
            _dragging.Start = Math.Clamp(_dragOriginalBeat  + deltaBeat,  0,        BeatCount - 1);
            InvalidateVisual();
            return;
        }

        // Curseur dynamique au survol
        Cursor = ResizeHitTest(pos) != null
            ? new Cursor(StandardCursorType.SizeWestEast)
            : HitTest(pos) != null
                ? new Cursor(StandardCursorType.SizeAll)
                : Cursor.Default;
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

        if (_resizing != null)
        {
            var note = _resizing;
            _resizing = null;
            e.Pointer.Capture(null);

            if (note.Length != _resizeOriginalLength)
                NoteResized?.Invoke(this, new NoteResizedEventArgs { NoteId = note.Id, NewLength = note.Length });

            InvalidateVisual();
            return;
        }

        if (_dragging != null)
        {
            var moved = _dragging;
            _dragging = null;
            e.Pointer.Capture(null);

            if (moved.Pitch != _dragOriginalPitch || moved.Start != _dragOriginalBeat)
                NoteMoved?.Invoke(this, new NoteMovedEventArgs { NoteId = moved.Id, NewPitch = moved.Pitch, NewBeat = moved.Start });

            InvalidateVisual();
        }
    }

    // ─── Render ──────────────────────────────────────────────────────────────

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

            bool isActive = note == _dragging || note == _resizing;
            ctx.FillRectangle(
                new SolidColorBrush(isActive ? Color.Parse("#81C784") : Color.Parse("#4CAF50")),
                new Rect(nx, ny, nw, nh));
            ctx.DrawRectangle(
                new Pen(new SolidColorBrush(Color.Parse("#A5D6A7")), 1),
                new Rect(nx, ny, nw, nh));

            // Handle resize (moitié droite)
            double halfW = nw / 2.0;
            ctx.FillRectangle(
                new SolidColorBrush(Color.Parse("#2E7D32")),
                new Rect(nx + halfW, ny, halfW, nh));
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
