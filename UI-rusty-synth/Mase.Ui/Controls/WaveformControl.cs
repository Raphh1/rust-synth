using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Media;
using Mase.Ui.Models;
using System;

namespace Mase.Ui.Controls;

public enum WaveformMode { View, Edit }

public class WaveformControl : Control
{
    public double[]     Table { get; set; } = Waveforms.Sine();
    public WaveformMode Mode  { get; set; } = WaveformMode.View;

    public event Action<double[]>? TableChanged;

    // ─── Edit interaction ────────────────────────────────────────────────────

    private bool   _pressing;
    private Point? _lastEditPos;

    private void ApplyEdit(Point pos)
    {
        double w = Bounds.Width;
        double h = Bounds.Height;
        if (w <= 0 || h <= 0 || Table.Length == 0) return;

        int    idx = Math.Clamp((int)(pos.X / w * Table.Length), 0, Table.Length - 1);
        double val = Math.Clamp(1.0 - 2.0 * pos.Y / h, -1.0, 1.0);

        if (_lastEditPos.HasValue)
        {
            int    prevIdx = Math.Clamp((int)(_lastEditPos.Value.X / w * Table.Length), 0, Table.Length - 1);
            double prevVal = Math.Clamp(1.0 - 2.0 * _lastEditPos.Value.Y / h, -1.0, 1.0);
            int    start   = Math.Min(prevIdx, idx);
            int    end     = Math.Max(prevIdx, idx);

            for (int i = start; i <= end; i++)
            {
                double t  = (end == start) ? 0.0 : (double)(i - start) / (end - start);
                if (idx < prevIdx) t = 1.0 - t;
                // Cubic ease in-out for smoother stroke feel
                double smooth = t * t * (3.0 - 2.0 * t);
                Table[i] = prevVal + (val - prevVal) * smooth;
            }
        }
        else
        {
            Table[idx] = val;
        }

        _lastEditPos = pos;
        InvalidateVisual();
    }

    protected override void OnPointerPressed(PointerPressedEventArgs e)
    {
        base.OnPointerPressed(e);
        if (Mode != WaveformMode.Edit) return;
        _pressing    = true;
        _lastEditPos = null;
        e.Pointer.Capture(this);
        Cursor = new Cursor(StandardCursorType.Cross);
        ApplyEdit(e.GetPosition(this));
    }

    protected override void OnPointerMoved(PointerEventArgs e)
    {
        base.OnPointerMoved(e);
        if (!_pressing || Mode != WaveformMode.Edit) return;
        ApplyEdit(e.GetPosition(this));
    }

    protected override void OnPointerReleased(PointerReleasedEventArgs e)
    {
        base.OnPointerReleased(e);
        if (!_pressing) return;
        _pressing    = false;
        _lastEditPos = null;
        e.Pointer.Capture(null);
        Cursor = Cursor.Default;
        if (Mode == WaveformMode.Edit)
            TableChanged?.Invoke((double[])Table.Clone());
    }

    // ─── Render ──────────────────────────────────────────────────────────────

    public override void Render(DrawingContext ctx)
    {
        double w = Bounds.Width;
        double h = Bounds.Height;
        if (w <= 0 || h <= 0 || Table.Length == 0) return;

        double mid    = h / 2.0;
        bool   isEdit = Mode == WaveformMode.Edit;
        int    n      = Table.Length;

        // Background
        ctx.FillRectangle(
            new SolidColorBrush(isEdit ? Color.Parse("#0D1117") : Color.Parse("#1A1A1A")),
            new Rect(0, 0, w, h));

        // Grid lines
        var gridPen = new Pen(new SolidColorBrush(Color.Parse("#2C2C2C")), 0.5);
        ctx.DrawLine(gridPen, new Point(0, mid),           new Point(w, mid));
        ctx.DrawLine(gridPen, new Point(0, mid - h / 4.0), new Point(w, mid - h / 4.0));
        ctx.DrawLine(gridPen, new Point(0, mid + h / 4.0), new Point(w, mid + h / 4.0));

        // Sample bars (View = vert sombre, Edit = bleu sombre)
        {
            var barBrush = new SolidColorBrush(isEdit ? Color.Parse("#1C3A5E") : Color.Parse("#1A3A1A"));
            double barW  = w / n;
            for (int i = 0; i < n; i++)
            {
                double v    = Math.Clamp(Table[i], -1.0, 1.0);
                double barH = Math.Abs(v) * mid;
                double barY = v >= 0 ? mid - barH : mid;
                ctx.FillRectangle(barBrush, new Rect(i * barW, barY, Math.Max(barW - 0.5, 1.0), barH));
            }
        }

        // Waveform curve — Catmull-Rom smooth rendering
        var curvePen = new Pen(
            new SolidColorBrush(isEdit ? Color.Parse("#42A5F5") : Color.Parse("#4CAF50")),
            1.5);

        const int steps = 4; // sub-samples per segment for smooth curve
        for (int i = 0; i < n - 1; i++)
        {
            double p0 = Table[Math.Max(i - 1, 0)];
            double p1 = Table[i];
            double p2 = Table[i + 1];
            double p3 = Table[Math.Min(i + 2, n - 1)];

            for (int s = 0; s < steps; s++)
            {
                double t0 = (double)s       / steps;
                double t1 = (double)(s + 1) / steps;
                double v0 = CatmullRom(p0, p1, p2, p3, t0);
                double v1 = CatmullRom(p0, p1, p2, p3, t1);

                double x0 = (i + t0) * w / (n - 1);
                double x1 = (i + t1) * w / (n - 1);
                double y0 = mid - Math.Clamp(v0, -1.0, 1.0) * mid;
                double y1 = mid - Math.Clamp(v1, -1.0, 1.0) * mid;

                ctx.DrawLine(curvePen, new Point(x0, y0), new Point(x1, y1));
            }
        }

        // Edit mode border
        if (isEdit)
            ctx.DrawRectangle(
                new Pen(new SolidColorBrush(Color.Parse("#1565C0")), 1.5),
                new Rect(0.75, 0.75, w - 1.5, h - 1.5));
    }

    private static double CatmullRom(double p0, double p1, double p2, double p3, double t)
    {
        double t2 = t * t;
        double t3 = t2 * t;
        return 0.5 * ((2 * p1)
            + (-p0 + p2) * t
            + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t2
            + (-p0 + 3 * p1 - 3 * p2 + p3) * t3);
    }
}
