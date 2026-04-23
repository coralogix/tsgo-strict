import type { Type320 } from '../batch-01/file-0320.js';
export interface Type321 {
  id: 321;
  name: 'File321';
  next: Type320;
}

export function make321(): Type321 {
  return { id: 321, name: 'File321', next: null as unknown as Type320 };
}
