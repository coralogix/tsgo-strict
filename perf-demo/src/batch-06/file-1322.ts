import type { Type1321 } from '../batch-06/file-1321.js';
export interface Type1322 {
  id: 1322;
  name: 'File1322';
  next: Type1321;
}

export function make1322(): Type1322 {
  return { id: 1322, name: 'File1322', next: null as unknown as Type1321 };
}
