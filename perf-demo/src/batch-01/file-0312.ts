import type { Type311 } from '../batch-01/file-0311.js';
export interface Type312 {
  id: 312;
  name: 'File312';
  next: Type311;
}

export function make312(): Type312 {
  return { id: 312, name: 'File312', next: null as unknown as Type311 };
}
