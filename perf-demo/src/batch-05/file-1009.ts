import type { Type1008 } from '../batch-05/file-1008.js';
export interface Type1009 {
  id: 1009;
  name: 'File1009';
  next: Type1008;
}

export function make1009(): Type1009 {
  return { id: 1009, name: 'File1009', next: null as unknown as Type1008 };
}
