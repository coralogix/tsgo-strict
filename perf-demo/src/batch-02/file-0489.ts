import type { Type488 } from '../batch-02/file-0488.js';
export interface Type489 {
  id: 489;
  name: 'File489';
  next: Type488;
}

export function make489(): Type489 {
  return { id: 489, name: 'File489', next: null as unknown as Type488 };
}
