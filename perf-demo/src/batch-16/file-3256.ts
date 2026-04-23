import type { Type3255 } from '../batch-16/file-3255.js';
export interface Type3256 {
  id: 3256;
  name: 'File3256';
  next: Type3255;
}

export function make3256(): Type3256 {
  return { id: 3256, name: 'File3256', next: null as unknown as Type3255 };
}
