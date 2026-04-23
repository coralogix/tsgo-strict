import type { Type1959 } from '../batch-09/file-1959.js';
export interface Type1960 {
  id: 1960;
  name: 'File1960';
  next: Type1959;
}

export function make1960(): Type1960 {
  return { id: 1960, name: 'File1960', next: null as unknown as Type1959 };
}
