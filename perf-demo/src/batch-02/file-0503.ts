import type { Type502 } from '../batch-02/file-0502.js';
export interface Type503 {
  id: 503;
  name: 'File503';
  next: Type502;
}

export function make503(): Type503 {
  return { id: 503, name: 'File503', next: null as unknown as Type502 };
}
