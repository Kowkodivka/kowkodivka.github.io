import { Component, onCleanup, onMount } from "solid-js";
import avatar from "@/assets/avatar.jpg";

interface MouseState {
  x: number;
  y: number;
  active: boolean;
}

interface Color {
  r: number;
  g: number;
  b: number;
}

const CONFIG = {
  PARTICLE_COUNT: 200,
  LINK_RADIUS: 150,
  MOUSE_INTERACT_RADIUS: 250,
  MOUSE_LINK_RADIUS: 200,
  MAX_LINKS_PER_PARTICLE: 5,
};

const FALLBACK_COLORS: Color[] = [
  { r: 60, g: 60, b: 60 },
  { r: 110, g: 110, b: 110 },
  { r: 170, g: 170, b: 170 },
  { r: 220, g: 220, b: 220 },
  { r: 255, g: 255, b: 255 },
];

export function extractPalette(src: string, colorCount = 5): Promise<Color[]> {
  return new Promise((resolve) => {
    const img = new Image();
    img.src = src;

    img.onload = () => {
      const size = 30;
      const canvas = document.createElement("canvas");

      canvas.width = size;
      canvas.height = size;

      const ctx = canvas.getContext("2d", { willReadFrequently: true });
      if (!ctx) return resolve(FALLBACK_COLORS.slice(0, colorCount));

      ctx.drawImage(img, 0, 0, size, size);
      const { data } = ctx.getImageData(0, 0, size, size);

      const colorMap = new Map<number, Color & { count: number }>();

      for (let i = 0; i < data.length; i += 4) {
        const r = data[i],
          g = data[i + 1],
          b = data[i + 2],
          a = data[i + 3];

        if (a < 128) continue;

        const key = ((r & 0xf0) << 16) | ((g & 0xf0) << 8) | (b & 0xf0);

        const existing = colorMap.get(key);

        if (existing) {
          existing.count++;
        } else {
          colorMap.set(key, { r, g, b, count: 1 });
        }
      }

      const palette = Array.from(colorMap.values())
        .sort((a, b) => b.count - a.count)
        .slice(0, colorCount)
        .map(({ r, g, b }) => ({ r, g, b }));

      while (palette.length < colorCount) {
        palette.push(FALLBACK_COLORS[palette.length % FALLBACK_COLORS.length]);
      }

      resolve(palette);
    };

    img.onerror = () => {
      resolve(FALLBACK_COLORS.slice(0, colorCount));
    };
  });
}

class Particle {
  x: number = 0;
  y: number = 0;
  size: number = 0;
  baseSize: number = 0;
  vx: number = 0;
  vy: number = 0;
  color: Color;
  alpha: number = 0;
  baseAlpha: number = 0;
  phase: number = 0;
  phaseSpeed: number = 0;
  glow: number = 0;

  constructor(
    width: number,
    height: number,
    initialColors: Color[],
    x?: number,
    y?: number,
  ) {
    this.color =
      initialColors[Math.floor(Math.random() * initialColors.length)];
    this.init(width, height, x, y);
  }

  init(width: number, height: number, startX?: number, startY?: number) {
    this.x = startX ?? Math.random() * width;
    this.y = startY ?? Math.random() * height;
    this.size = Math.random() * 2.8 + 0.8;
    this.baseSize = this.size;
    this.vx = (Math.random() - 0.5) * 0.25;
    this.vy = (Math.random() - 0.5) * 0.2;
    this.alpha = Math.random() * 0.55 + 0.15;
    this.baseAlpha = this.alpha;
    this.phase = Math.random() * Math.PI * 2;
    this.phaseSpeed = Math.random() * 0.02 + 0.005;
    this.glow = this.size * (4 + Math.random() * 5);
  }

  update(width: number, height: number, mouse: MouseState) {
    this.phase += this.phaseSpeed;
    this.x += this.vx + Math.sin(this.phase * 0.6) * 0.12;
    this.y += this.vy + Math.cos(this.phase * 0.8) * 0.08;
    this.alpha = this.baseAlpha + Math.sin(this.phase) * 0.18;
    this.size = this.baseSize + Math.sin(this.phase * 1.2) * 0.7;

    if (mouse.active) {
      const dx = mouse.x - this.x;
      const dy = mouse.y - this.y;
      const distSq = dx * dx + dy * dy;
      const interactRadiusSq = CONFIG.MOUSE_INTERACT_RADIUS ** 2;

      if (distSq < interactRadiusSq) {
        const d = Math.sqrt(distSq);
        const force =
          (CONFIG.MOUSE_INTERACT_RADIUS - d) / CONFIG.MOUSE_INTERACT_RADIUS;

        this.vx += (dx / d) * force * 0.06;
        this.vy += (dy / d) * force * 0.06;
        this.alpha = Math.min(1, this.baseAlpha + force * 0.6);
        this.size = this.baseSize + force * 4;
        this.glow = (this.baseSize + force * 4) * (5 + force * 8);
      }
    }

    this.vx *= 0.985;
    this.vy *= 0.985;

    const padding = 40;
    if (this.x < -padding) this.x = width + padding;
    if (this.x > width + padding) this.x = -padding;
    if (this.y < -padding) this.y = height + padding;
    if (this.y > height + padding) this.y = -padding;
  }

  draw(ctx: CanvasRenderingContext2D) {
    const currentAlpha = Math.max(0, this.alpha);
    const { r, g, b } = this.color;

    const gradient = ctx.createRadialGradient(
      this.x,
      this.y,
      0,
      this.x,
      this.y,
      this.glow,
    );
    gradient.addColorStop(0, `rgba(${r},${g},${b},${currentAlpha * 0.4})`);
    gradient.addColorStop(0.3, `rgba(${r},${g},${b},${currentAlpha * 0.1})`);
    gradient.addColorStop(1, `rgba(${r},${g},${b},0)`);

    ctx.fillStyle = gradient;
    ctx.fillRect(
      this.x - this.glow,
      this.y - this.glow,
      this.glow * 2,
      this.glow * 2,
    );

    ctx.beginPath();
    ctx.arc(this.x, this.y, Math.max(0.3, this.size), 0, Math.PI * 2);
    ctx.fillStyle = `rgba(${r},${g},${b},${currentAlpha})`;
    ctx.fill();
  }
}

const BackgroundParticles: Component = () => {
  let canvasRef!: HTMLCanvasElement;

  onMount(() => {
    const canvas = canvasRef;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const isTouchDevice = (): boolean => {
      if (navigator.maxTouchPoints > 0) return true;
      return false;
    };

    const isTouch = isTouchDevice();

    let width: number;
    let height: number;
    let animationFrameId: number;
    let resizeTimer: ReturnType<typeof setTimeout>;

    const mouse: MouseState = { x: -9999, y: -9999, active: false };
    let particles: Particle[] = [];
    let currentPalette = FALLBACK_COLORS;

    const resizeCanvas = () => {
      width = canvas.width = document.documentElement.clientWidth;
      height = canvas.height = document.documentElement.clientHeight;

      particles = [];
      for (let i = 0; i < CONFIG.PARTICLE_COUNT; i++) {
        particles.push(new Particle(width, height, currentPalette));
      }
    };

    const handleResize = () => {
      clearTimeout(resizeTimer);
      resizeTimer = setTimeout(resizeCanvas, 100);
    };

    const handleMouseMove = (e: MouseEvent) => {
      mouse.x = e.clientX;
      mouse.y = e.clientY;
      mouse.active = true;
    };

    const handleMouseLeave = () => {
      mouse.active = false;
    };

    const drawLines = () => {
      const linkRadiusSq = CONFIG.LINK_RADIUS ** 2;
      const mouseLinkRadiusSq = CONFIG.MOUSE_LINK_RADIUS ** 2;

      for (let i = 0; i < particles.length; i++) {
        let links = 0;
        const p1 = particles[i];

        for (let j = i + 1; j < particles.length; j++) {
          const p2 = particles[j];
          const dx = p1.x - p2.x;
          const dy = p1.y - p2.y;
          const distSq = dx * dx + dy * dy;

          if (distSq < linkRadiusSq) {
            links++;
            if (links > CONFIG.MAX_LINKS_PER_PARTICLE) break;

            const d = Math.sqrt(distSq);
            const lineAlpha = (1 - d / CONFIG.LINK_RADIUS) * 0.15;

            ctx.beginPath();
            ctx.moveTo(p1.x, p1.y);
            ctx.lineTo(p2.x, p2.y);
            ctx.strokeStyle = `rgba(${p1.color.r}, ${p1.color.g}, ${p1.color.b}, ${lineAlpha})`;
            ctx.lineWidth = 0.5;
            ctx.stroke();
          }
        }
      }

      if (mouse.active) {
        for (let i = 0; i < particles.length; i++) {
          const p = particles[i];
          const dx = mouse.x - p.x;
          const dy = mouse.y - p.y;
          const distSq = dx * dx + dy * dy;

          if (distSq < mouseLinkRadiusSq) {
            const d = Math.sqrt(distSq);
            const lineAlpha = (1 - d / CONFIG.MOUSE_LINK_RADIUS) * 0.18;

            ctx.beginPath();
            ctx.moveTo(p.x, p.y);
            ctx.lineTo(mouse.x, mouse.y);
            ctx.strokeStyle = `rgba(${p.color.r}, ${p.color.g}, ${p.color.b}, ${lineAlpha})`;
            ctx.lineWidth = 0.6;
            ctx.stroke();
          }
        }
      }
    };

    const loop = () => {
      ctx.clearRect(0, 0, width, height);
      particles.forEach((p) => {
        p.update(width, height, mouse);
        p.draw(ctx);
      });
      drawLines();
      animationFrameId = requestAnimationFrame(loop);
    };

    resizeCanvas();

    window.addEventListener("resize", handleResize);

    if (!isTouch) {
      document.addEventListener("mousemove", handleMouseMove);
      document.addEventListener("mouseleave", handleMouseLeave);
    }

    loop();

    extractPalette(avatar).then((extractedColors) => {
      currentPalette = extractedColors;
      particles.forEach((p) => {
        p.color =
          currentPalette[Math.floor(Math.random() * currentPalette.length)];
      });
    });

    onCleanup(() => {
      window.removeEventListener("resize", handleResize);

      if (!isTouch) {
        document.removeEventListener("mousemove", handleMouseMove);
        document.removeEventListener("mouseleave", handleMouseLeave);
      }

      clearTimeout(resizeTimer);
      cancelAnimationFrame(animationFrameId);
    });
  });

  return (
    <canvas
      ref={canvasRef}
      class="fixed inset-0 h-screen w-screen pointer-events-none z-[-1]"
    />
  );
};

export default BackgroundParticles;
