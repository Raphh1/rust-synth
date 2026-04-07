using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Media;
using System;

namespace Mase.Ui.Controls;

public class EnvelopeControl : Control
{
    private double _a = 0.01, _d = 0.1, _s = 0.7, _r = 0.3;

    public double A { get => _a; set { _a = value; InvalidateVisual(); } }
    public double D { get => _d; set { _d = value; InvalidateVisual(); } }
    public double S { get => _s; set { _s = value; InvalidateVisual(); } }
    public double R { get => _r; set { _r = value; InvalidateVisual(); } }

    public event Action<double, double, double, double>? EnvelopeChanged;

    private const double MaxA = 2.0, MaxD = 2.0, MaxR = 5.0;
    private const double SustainWidth = 0.15;
    private const double HitRadius = 10.0;

    // ─── Drag state ──────────────────────────────────────────────────────────

    private enum DragTarget { None, Attack, Decay, Release }
    private DragTarget _drag = DragTarget.None;

    // Layout snapshot at press time
    private double _gw, _gh, _pad, _top, _xA, _xD, _xS, _total;

    private void ComputeLayout(
        out double pad, out double gw, out double gh, out double top, out double bot,
        out double xA, out double xD, out double xS, out double xR,
        out double ySus, out double total)
    {
        double w = Bounds.Width, h = Bounds.Height;
        pad   = 4;
        gw    = w - pad * 2;
        gh    = h - pad * 2 - 14;
        top   = pad;
        bot   = pad + gh;
        total = MaxA + MaxD + MaxA * SustainWidth + MaxR;

        double aNorm   = (_a / total) * gw;
        double dNorm   = (_d / total) * gw;
        double susNorm = (MaxA * SustainWidth / total) * gw;
        double rNorm   = (_r / total) * gw;

        xA   = pad + aNorm;
        xD   = xA  + dNorm;
        xS   = xD  + susNorm;
        xR   = xS  + rNorm;
        ySus = top  + (1.0 - _s) * gh;
    }

    // ─── Pointer interaction ─────────────────────────────────────────────────

    protected override void OnPointerPressed(PointerPressedEventArgs e)
    {
        base.OnPointerPressed(e);
        if (!e.GetCurrentPoint(this).Properties.IsLeftButtonPressed) return;

        ComputeLayout(out double pad, out double gw, out double gh, out double top, out double bot,
                      out double xA, out double xD, out double xS, out double xR,
                      out double ySus, out double total);

        var pos = e.GetPosition(this);

        if      (Dist(pos, xD, ySus) <= HitRadius) _drag = DragTarget.Decay;
        else if (Dist(pos, xA, top)  <= HitRadius) _drag = DragTarget.Attack;
        else if (Dist(pos, xR, bot)  <= HitRadius) _drag = DragTarget.Release;
        else return;

        _gw = gw; _gh = gh; _pad = pad; _top = top;
        _xA = xA; _xD = xD; _xS = xS; _total = total;

        e.Pointer.Capture(this);
        Cursor = new Cursor(StandardCursorType.SizeAll);
    }

    protected override void OnPointerMoved(PointerEventArgs e)
    {
        base.OnPointerMoved(e);

        if (_drag == DragTarget.None)
        {
            ComputeLayout(out _, out _, out _, out double top, out double bot,
                          out double xA, out double xD, out double xS, out double xR,
                          out double ySus, out _);
            var pos = e.GetPosition(this);
            Cursor = (Dist(pos, xD, ySus) <= HitRadius ||
                      Dist(pos, xA, top)  <= HitRadius ||
                      Dist(pos, xR, bot)  <= HitRadius)
                ? new Cursor(StandardCursorType.Hand)
                : Cursor.Default;
            return;
        }

        var p = e.GetPosition(this);
        switch (_drag)
        {
            case DragTarget.Attack:
            {
                double nx = Math.Clamp(p.X, _pad + 1, _xD - 1);
                _a = Math.Clamp((nx - _pad) / _gw * _total, 0.001, MaxA);
                break;
            }
            case DragTarget.Decay:
            {
                double nx = Math.Clamp(p.X, _xA + 1, _xS - 1);
                _d = Math.Clamp((nx - _xA) / _gw * _total, 0.001, MaxD);
                _s = Math.Clamp(1.0 - (p.Y - _top) / _gh, 0.0, 1.0);
                break;
            }
            case DragTarget.Release:
            {
                double nx = Math.Clamp(p.X, _xS + 1, _pad + _gw);
                _r = Math.Clamp((nx - _xS) / _gw * _total, 0.001, MaxR);
                break;
            }
        }

        InvalidateVisual();
    }

    protected override void OnPointerReleased(PointerReleasedEventArgs e)
    {
        base.OnPointerReleased(e);
        if (_drag == DragTarget.None) return;

        _drag = DragTarget.None;
        e.Pointer.Capture(null);
        Cursor = Cursor.Default;
        EnvelopeChanged?.Invoke(_a, _d, _s, _r);
    }

    private static double Dist(Point p, double x, double y)
        => Math.Sqrt((p.X - x) * (p.X - x) + (p.Y - y) * (p.Y - y));

    // ─── Render ──────────────────────────────────────────────────────────────

    public override void Render(DrawingContext ctx)
    {
        double w = Bounds.Width;
        double h = Bounds.Height;
        if (w <= 0 || h <= 0) return;

        ComputeLayout(out double pad, out double gw, out double gh, out double top, out double bot,
                      out double xA, out double xD, out double xS, out double xR,
                      out double ySus, out _);

        // Background
        ctx.FillRectangle(new SolidColorBrush(Color.Parse("#141414")), new Rect(0, 0, w, h));

        // Fill
        var fillGeom = new PathGeometry();
        var fig      = new PathFigure { StartPoint = new Point(pad, bot), IsClosed = true };
        fig.Segments!.Add(new LineSegment { Point = new Point(xA, top) });
        fig.Segments.Add(new LineSegment  { Point = new Point(xD, ySus) });
        fig.Segments.Add(new LineSegment  { Point = new Point(xS, ySus) });
        fig.Segments.Add(new LineSegment  { Point = new Point(xR, bot) });
        fillGeom.Figures.Add(fig);
        ctx.DrawGeometry(new SolidColorBrush(Color.Parse("#1A3A1A")), null, fillGeom);

        // Curve
        var pen = new Pen(new SolidColorBrush(Color.Parse("#4CAF50")), 1.5);
        ctx.DrawLine(pen, new Point(pad, bot), new Point(xA, top));
        ctx.DrawLine(pen, new Point(xA, top),  new Point(xD, ySus));
        ctx.DrawLine(pen, new Point(xD, ySus), new Point(xS, ySus));
        ctx.DrawLine(pen, new Point(xS, ySus), new Point(xR, bot));

        // Control dots — highlighted when dragging
        var dotBrush     = new SolidColorBrush(Color.Parse("#81C784"));
        var dotActiveBrush = new SolidColorBrush(Color.Parse("#FFFFFF"));
        DrawDot(ctx, _drag == DragTarget.Attack  ? dotActiveBrush : dotBrush, xA, top);
        DrawDot(ctx, _drag == DragTarget.Decay   ? dotActiveBrush : dotBrush, xD, ySus);
        DrawDot(ctx, _drag == DragTarget.Release ? dotActiveBrush : dotBrush, xR, bot);

        // Labels
        var typeface   = new Typeface("Inter");
        var labelBrush = new SolidColorBrush(Color.Parse("#888888"));
        DrawLabel(ctx, typeface, labelBrush, "A", (pad + xA) / 2, bot + 3);
        DrawLabel(ctx, typeface, labelBrush, "D", (xA + xD) / 2,  bot + 3);
        DrawLabel(ctx, typeface, labelBrush, "S", (xD + xS) / 2,  bot + 3);
        DrawLabel(ctx, typeface, labelBrush, "R", (xS + xR) / 2,  bot + 3);
    }

    private static void DrawDot(DrawingContext ctx, IBrush brush, double x, double y)
        => ctx.FillRectangle(brush, new Rect(x - 3, y - 3, 6, 6));

    private static void DrawLabel(DrawingContext ctx, Typeface tf, IBrush brush, string text, double x, double y)
    {
        var ft = new Avalonia.Media.FormattedText(
            text,
            System.Globalization.CultureInfo.InvariantCulture,
            Avalonia.Media.FlowDirection.LeftToRight,
            tf, 9, brush);
        ctx.DrawText(ft, new Point(x - ft.Width / 2, y));
    }
}
