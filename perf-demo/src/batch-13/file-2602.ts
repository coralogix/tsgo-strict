import type { Type2601 } from '../batch-13/file-2601.js';
export interface Type2602 {
  id: 2602;
  name: 'File2602';
  next: Type2601;
}

export function make2602(): Type2602 {
  return { id: 2602, name: 'File2602', next: null as unknown as Type2601 };
}
