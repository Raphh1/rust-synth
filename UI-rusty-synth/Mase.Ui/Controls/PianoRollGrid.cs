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

public class PianoRollGrid : Control
{
    public int BaseNote  { get; set; } = 36; // C2
    public int NoteCount { get; set; } = 48; // 4 octaves

    public List<NoteModel> Notes { get; set; } = new();

    public event EventHandler<NoteAddedEventArgs>? NoteAdded;

    private const int    BeatCount   = 16;
    private const int    BeatsPerBar = 4;
    private const double LabelWidth  = 36;

    private static readonly bool[] IsBlackKey =
        { false, true, false, true, false, false, true, false, true, false, true, false };

    protected override void OnPointerPressed(PointerPressedEventArgs e)
    {
        base.OnPointerPressed(e);

        var pos  = e.GetPosition(this);
        if (pos.X <= LabelWidth) return;

        double rowH  = Bounds.Height / NoteCount;
        double gridW = Bounds.Width - LabelWidth;
        double beatW = gridW / BeatCount;

        int rowIndex = (int)(pos.Y / rowH);
        int beat     = (int)((pos.X - LabelWidth) / beatW);

        rowIndex = Math.Clamp(rowIndex, 0, NoteCount - 1);
        beat     = Math.Clamp(beat, 0, BeatCount - 1);

        int pitch = BaseNote + (NoteCount - 1 - rowIndex);

        NoteAdded?.Invoke(this, new NoteAddedEventArgs { Pitch = pitch, Beat = beat });
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

        // Background
        ctx.FillRectangle(new SolidColorBrush(Color.Parse("#141414")), new Rect(0, 0, w, h));

        // === Rows ===
        for (int i = 0; i < NoteCount; i++)
        {
            int    midi     = BaseNote + (NoteCount - 1 - i);
            int    semitone = midi % 12;
            double y        = i * rowH;

            var rowBrush = IsBlackKey[semitone]
                ? new SolidColorBrush(Color.Parse("#0F0F0F"))
                : new SolidColorBrush(Color.Parse("#1C1C1C"));

            ctx.FillRectangle(rowBrush, new Rect(LabelWidth, y, gridW, rowH));

            if (semitone == 0)
                ctx.DrawLine(
                    new Pen(new SolidColorBrush(Color.Parse("#505050")), 1),
                    new Point(LabelWidth, y), new Point(w, y));
            else
                ctx.DrawLine(
                    new Pen(new SolidColorBrush(Color.Parse("#272727")), 0.5),
                    new Point(LabelWidth, y + rowH), new Point(w, y + rowH));
        }

        // === Vertical lines ===
        for (int b = 0; b <= BeatCount; b++)
        {
            double x     = LabelWidth + b * beatW;
            bool   isBar = b % BeatsPerBar == 0;
            ctx.DrawLine(
                isBar
                    ? new Pen(new SolidColorBrush(Color.Parse("#585858")), 1)
                    : new Pen(new SolidColorBrush(Color.Parse("#2A2A2A")), 0.5),
                new Point(x, 0), new Point(x, h));
        }

        // === Notes ===
        var noteBrush  = new SolidColorBrush(Color.Parse("#4CAF50"));
        var noteBorder = new Pen(new SolidColorBrush(Color.Parse("#81C784")), 1);
        foreach (var note in Notes)
        {
            int rowIndex = NoteCount - 1 - (note.Pitch - BaseNote);
            if (rowIndex < 0 || rowIndex >= NoteCount) continue;

            double nx = LabelWidth + note.Start * beatW;
            double ny = rowIndex * rowH;
            double nw = note.Length * beatW - 1;
            double nh = rowH - 1;

            ctx.FillRectangle(noteBrush, new Rect(nx, ny, nw, nh));
            ctx.DrawRectangle(noteBorder, new Rect(nx, ny, nw, nh));
        }

        // === Label panel ===
        ctx.FillRectangle(new SolidColorBrush(Color.Parse("#1A1A1A")), new Rect(0, 0, LabelWidth, h));
        ctx.DrawLine(
            new Pen(new SolidColorBrush(Color.Parse("#3E3E42")), 1),
            new Point(LabelWidth, 0), new Point(LabelWidth, h));

        var typeface = new Typeface("Inter");
        for (int i = 0; i < NoteCount; i++)
        {
            int    midi     = BaseNote + (NoteCount - 1 - i);
            int    semitone = midi % 12;
            if (semitone != 0) continue;

            int    octave = midi / 12 - 1;
            double y      = i * rowH;

            var ft = new FormattedText(
                $"C{octave}",
                CultureInfo.InvariantCulture,
                FlowDirection.LeftToRight,
                typeface,
                fontSize,
                new SolidColorBrush(Color.Parse("#BBBBBB")));

            double labelY = Math.Clamp(y + rowH / 2 - ft.Height / 2, 0, h - ft.Height);
            ctx.DrawText(ft, new Point(4, labelY));
        }
    }
}
