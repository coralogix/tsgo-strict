import type { Type410 } from '../batch-02/file-0410.js';
export interface Type411 {
  id: 411;
  name: 'File411';
  next: Type410;
}

export function make411(): Type411 {
  return { id: 411, name: 'File411', next: null as unknown as Type410 };
}
