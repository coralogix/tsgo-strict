import type { Type254 } from '../batch-01/file-0254.js';
export interface Type255 {
  id: 255;
  name: 'File255';
  next: Type254;
}

export function make255(): Type255 {
  return { id: 255, name: 'File255', next: null as unknown as Type254 };
}
