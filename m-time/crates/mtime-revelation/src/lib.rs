#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum InterpretationCategory{Text,ClassicalCommentary,ModernInterpretation,ConceptualCorrelation}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct TextualReference{pub id:&'static str,pub corpus:&'static str,pub concepts:&'static [&'static str],pub category:InterpretationCategory}
pub const REFERENCES:&[TextualReference]=&[
 TextualReference{id:"Quran 2:189",corpus:"Quran",concepts:&["hilal","mawaqit","people","Hajj"],category:InterpretationCategory::Text},
 TextualReference{id:"Quran 10:5",corpus:"Quran",concepts:&["Sun","Moon","manazil","years","hisab"],category:InterpretationCategory::Text},
 TextualReference{id:"Quran 55:5",corpus:"Quran",concepts:&["Sun","Moon","reckoning"],category:InterpretationCategory::Text},
 TextualReference{id:"Quran 21:33",corpus:"Quran",concepts:&["Sun","Moon","falak"],category:InterpretationCategory::Text},
 TextualReference{id:"Genesis 1:14",corpus:"Torah/Hebrew Bible witness",concepts:&["luminaries","signs","appointed times","days","years"],category:InterpretationCategory::Text},
 TextualReference{id:"Psalms 104:19",corpus:"Psalms witness",concepts:&["Moon","appointed times"],category:InterpretationCategory::Text},
 TextualReference{id:"Mark 13:32",corpus:"Gospel witness",concepts:&["day","hour","epistemic limit"],category:InterpretationCategory::Text},
];
pub const NUMERICAL_INFLUENCE:bool=false;
#[cfg(test)]
mod tests{use super::*;#[test]fn revelation_is_non_numerical(){assert!(!NUMERICAL_INFLUENCE);assert!(REFERENCES.len()>=7);}}
