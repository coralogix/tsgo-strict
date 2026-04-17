import type { Type3700 } from '../batch-18/file-3700.js';
export interface Type3701 {
  id: 3701;
  name: 'File3701';
  next: Type3700;
}

export function make3701(): Type3701 {
  return { id: 3701, name: 'File3701', next: null as unknown as Type3700 };
}
