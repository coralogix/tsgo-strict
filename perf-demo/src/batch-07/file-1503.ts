import type { Type1502 } from '../batch-07/file-1502.js';
export interface Type1503 {
  id: 1503;
  name: 'File1503';
  next: Type1502;
}

export function make1503(): Type1503 {
  return { id: 1503, name: 'File1503', next: null as unknown as Type1502 };
}
