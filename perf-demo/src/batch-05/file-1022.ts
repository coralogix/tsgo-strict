import type { Type1021 } from '../batch-05/file-1021.js';
export interface Type1022 {
  id: 1022;
  name: 'File1022';
  next: Type1021;
}

export function make1022(): Type1022 {
  return { id: 1022, name: 'File1022', next: null as unknown as Type1021 };
}
