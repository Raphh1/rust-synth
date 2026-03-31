using Avalonia;
using Avalonia.Controls;
using Avalonia.Media;
using System;
using System.Globalization;

namespace Mase.Ui.Controls;

public class PianoRollGrid : Control
{
    // Configurable range
    public int BaseNote  { get; set; } = 36; // C2 MIDI
    public int NoteCount { get; set; } = 48; // 4 octaves

    private const int    BeatCount   = 16;
    private const int    BeatsPerBar = 4;
    private const double LabelWidth  = 36;

    private static readonly bool[] IsBlackKey =
        { false, true, false, true, false, false, true, false, true, false, true, false };

    public override void Render(DrawingContext ctx)
    {
        double w = Bounds.Width;
        double h = Bounds.Height;
        if (w <= 0 || h <= 0) return;

        double rowH      = h / NoteCount;
        double octaveH   = rowH * 12;
        double fontSize  = Math.Clamp(octaveH * 0.25, 9, 16);
        double gridW = w - LabelWidth;
        double beatW = gridW / BeatCount;

        // Background
        ctx.FillRectangle(new SolidColorBrush(Color.Parse("#141414")), new Rect(0, 0, w, h));

        // === Rows (high pitch → top) ===
        for (int i = 0; i < NoteCount; i++)
        {
            int    midi     = BaseNote + (NoteCount - 1 - i);
            int    semitone = midi % 12;
            double y        = i * rowH;

            // Black keys noticeably darker
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

        // === Vertical lines (time) ===
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
