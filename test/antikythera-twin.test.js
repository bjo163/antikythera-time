import test from 'node:test';import assert from 'node:assert/strict';
import { ANTIKYTHERA_DIGITAL_TWIN,antikytheraComponent,antikytheraEvidenceSummary } from '../src/index.js';
test('digital twin never collapses all front planetary gearing into surviving evidence',()=>{assert.equal(antikytheraComponent('planet-inscriptions').status,'SURVIVING_EVIDENCE');assert.notEqual(antikytheraComponent('superior-planets-2021').status,'SURVIVING_EVIDENCE');});
test('digital twin includes surviving and reconstructed categories',()=>{const s=antikytheraEvidenceSummary();assert.ok(s.SURVIVING_EVIDENCE>=5);assert.ok(s.RECONSTRUCTED_MODEL>=1);});
test('digital twin boundary rejects unique-history overclaim',()=>assert.match(ANTIKYTHERA_DIGITAL_TWIN.boundary,/does not claim one unique/));
