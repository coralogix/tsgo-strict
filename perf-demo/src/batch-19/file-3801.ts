import type { Type3800 } from '../batch-19/file-3800.js';
export interface Type3801 {
  id: 3801;
  name: 'File3801';
  next: Type3800;
}

export function make3801(): Type3801 {
  return { id: 3801, name: 'File3801', next: null as unknown as Type3800 };
}
