import type { Type3132 } from '../batch-15/file-3132.js';
export interface Type3133 {
  id: 3133;
  name: 'File3133';
  next: Type3132;
}

export function make3133(): Type3133 {
  return { id: 3133, name: 'File3133', next: null as unknown as Type3132 };
}
