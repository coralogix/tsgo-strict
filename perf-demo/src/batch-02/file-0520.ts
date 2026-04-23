import type { Type519 } from '../batch-02/file-0519.js';
export interface Type520 {
  id: 520;
  name: 'File520';
  next: Type519;
}

export function make520(): Type520 {
  return { id: 520, name: 'File520', next: null as unknown as Type519 };
}
