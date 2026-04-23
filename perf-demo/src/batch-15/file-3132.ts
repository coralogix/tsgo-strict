import type { Type3131 } from '../batch-15/file-3131.js';
export interface Type3132 {
  id: 3132;
  name: 'File3132';
  next: Type3131;
}

export function make3132(): Type3132 {
  return { id: 3132, name: 'File3132', next: null as unknown as Type3131 };
}
