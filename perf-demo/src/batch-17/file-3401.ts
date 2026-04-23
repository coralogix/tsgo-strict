import type { Type3400 } from '../batch-17/file-3400.js';
export interface Type3401 {
  id: 3401;
  name: 'File3401';
  next: Type3400;
}

export function make3401(): Type3401 {
  return { id: 3401, name: 'File3401', next: null as unknown as Type3400 };
}
