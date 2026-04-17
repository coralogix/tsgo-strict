import type { Type1822 } from '../batch-09/file-1822.js';
export interface Type1823 {
  id: 1823;
  name: 'File1823';
  next: Type1822;
}

export function make1823(): Type1823 {
  return { id: 1823, name: 'File1823', next: null as unknown as Type1822 };
}
