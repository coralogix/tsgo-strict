import type { Type1330 } from '../batch-06/file-1330.js';
export interface Type1331 {
  id: 1331;
  name: 'File1331';
  next: Type1330;
}

export function make1331(): Type1331 {
  return { id: 1331, name: 'File1331', next: null as unknown as Type1330 };
}
