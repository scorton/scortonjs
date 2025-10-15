import { z } from 'zod';

export const ScanSchema = z.object({
  status: z.string().optional(),
  data: z.any().optional(),
  errors: z.any().optional(),
}).passthrough();

export const ScoreSchema = z.object({
  status: z.string().optional(),
  score: z.number().optional(),
  factors: z.any().optional(),
}).passthrough();

export function safeParseJSON<T = unknown>(text: string): { ok: boolean; data?: T; error?: string } {
  try {
    const obj = JSON.parse(text);
    return { ok: true, data: obj as T };
  } catch (e: any) {
    return { ok: false, error: e?.message ?? 'Invalid JSON' };
  }
}

