import type { Type3008 } from '../batch-15/file-3008.js';
export interface Type3009 {
  id: 3009;
  name: 'File3009';
  next: Type3008;
}

export function make3009(): Type3009 {
  return { id: 3009, name: 'File3009', next: null as unknown as Type3008 };
}
