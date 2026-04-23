import type { Type3280 } from '../batch-16/file-3280.js';
export interface Type3281 {
  id: 3281;
  name: 'File3281';
  next: Type3280;
}

export function make3281(): Type3281 {
  return { id: 3281, name: 'File3281', next: null as unknown as Type3280 };
}
