import type { Type405 } from '../batch-02/file-0405.js';
export interface Type406 {
  id: 406;
  name: 'File406';
  next: Type405;
}

export function make406(): Type406 {
  return { id: 406, name: 'File406', next: null as unknown as Type405 };
}
