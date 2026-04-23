import type { Type1708 } from '../batch-08/file-1708.js';
export interface Type1709 {
  id: 1709;
  name: 'File1709';
  next: Type1708;
}

export function make1709(): Type1709 {
  return { id: 1709, name: 'File1709', next: null as unknown as Type1708 };
}
