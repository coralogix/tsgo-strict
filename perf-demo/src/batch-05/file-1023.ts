import type { Type1022 } from '../batch-05/file-1022.js';
export interface Type1023 {
  id: 1023;
  name: 'File1023';
  next: Type1022;
}

export function make1023(): Type1023 {
  return { id: 1023, name: 'File1023', next: null as unknown as Type1022 };
}
