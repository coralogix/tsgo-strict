import type { Type370 } from '../batch-01/file-0370.js';
export interface Type371 {
  id: 371;
  name: 'File371';
  next: Type370;
}

export function make371(): Type371 {
  return { id: 371, name: 'File371', next: null as unknown as Type370 };
}
