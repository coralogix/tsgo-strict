import type { Type1060 } from '../batch-05/file-1060.js';
export interface Type1061 {
  id: 1061;
  name: 'File1061';
  next: Type1060;
}

export function make1061(): Type1061 {
  return { id: 1061, name: 'File1061', next: null as unknown as Type1060 };
}
