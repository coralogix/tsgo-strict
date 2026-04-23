import type { Type1601 } from '../batch-08/file-1601.js';
export interface Type1602 {
  id: 1602;
  name: 'File1602';
  next: Type1601;
}

export function make1602(): Type1602 {
  return { id: 1602, name: 'File1602', next: null as unknown as Type1601 };
}
