import type { Type310 } from '../batch-01/file-0310.js';
export interface Type311 {
  id: 311;
  name: 'File311';
  next: Type310;
}

export function make311(): Type311 {
  return { id: 311, name: 'File311', next: null as unknown as Type310 };
}
