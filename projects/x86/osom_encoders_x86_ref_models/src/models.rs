// Auto-generated from x86reference.xsd.
#![allow(unused)]
#![allow(dead_code)]
#![allow(clippy::all)]
use xsd_parser::{
    quick_xml::{
        DeserializeBytes, DeserializeReader, Error, ErrorKind, RawByteStr,
        WithDeserializer,
    },
    schema::Namespace,
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace =
    Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
#[derive(Debug)]
pub enum AddressType {
    Ba,
    Bb,
    Bd,
    X,
    Y,
    Sc,
    S2,
    S30,
    S33,
    F,
    I,
    Est,
}
impl DeserializeBytes for AddressType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"BA" => Ok(Self::Ba),
            b"BB" => Ok(Self::Bb),
            b"BD" => Ok(Self::Bd),
            b"X" => Ok(Self::X),
            b"Y" => Ok(Self::Y),
            b"SC" => Ok(Self::Sc),
            b"S2" => Ok(Self::S2),
            b"S30" => Ok(Self::S30),
            b"S33" => Ok(Self::S33),
            b"F" => Ok(Self::F),
            b"I" => Ok(Self::I),
            b"EST" => Ok(Self::Est),
            x => Err(reader
                .map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum OneOrZeroType {
    _1,
    _0,
}
impl DeserializeBytes for OneOrZeroType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"1" => Ok(Self::_1),
            b"0" => Ok(Self::_0),
            x => Err(reader
                .map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct EntryType {
    pub doc_part_alias_ref: Option<String>,
    pub part_alias: Option<String>,
    pub mod_: Option<EntryTypeModType>,
    pub fpush: Option<YesNoType>,
    pub fpop: Option<String>,
    pub mem_format: Option<String>,
    pub doc_64_ref: Option<String>,
    pub alias: Option<String>,
    pub tttn: Option<String>,
    pub sign_ext: Option<String>,
    pub particular: Option<YesNoType>,
    pub doc: Option<EntryTypeDocType>,
    pub doc_ref: Option<String>,
    pub is_doc: Option<YesNoType>,
    pub is_undoc: Option<YesNoType>,
    pub ring: Option<EntryTypeRingType>,
    pub ring_ref: Option<EntryTypeRingRefType>,
    pub ref_: Option<String>,
    pub mode: Option<EntryTypeModeType>,
    pub doc_1632_ref: Option<String>,
    pub r: Option<YesNoType>,
    pub attr: Option<EntryTypeAttrType>,
    pub lock: Option<YesNoType>,
    pub direction: Option<OneOrZeroType>,
    pub op_size: Option<OneOrZeroType>,
    pub pref: Option<String>,
    pub opcd_ext: Option<i32>,
    pub sec_opcd: Option<EntryTypeSecOpcdElementType>,
    pub proc_start: Option<EntryTypeProcStartElementType>,
    pub proc_end: Option<i32>,
    pub syntax: Vec<SyntaxType>,
    pub instr_ext: Option<EntryTypeInstrExtElementType>,
    pub grp_1: Option<String>,
    pub grp_2: Vec<String>,
    pub grp_3: Vec<String>,
    pub test_f: Option<String>,
    pub modif_f: Option<EntryTypeModifFElementType>,
    pub def_f: Option<EntryTypeDefFElementType>,
    pub undef_f: Option<String>,
    pub f_vals: Option<String>,
    pub modif_f_fpu: Option<String>,
    pub def_f_fpu: Option<String>,
    pub undef_f_fpu: Option<String>,
    pub f_vals_fpu: Option<String>,
    pub note: Option<SkipmeType>,
}
impl WithDeserializer for EntryType {
    type Deserializer = quick_xml_deserialize::EntryTypeDeserializer;
}
#[derive(Debug)]
pub struct GennotesType {
    pub gen_note: Vec<GennotesTypeGenNoteElementType>,
}
impl WithDeserializer for GennotesType {
    type Deserializer = quick_xml_deserialize::GennotesTypeDeserializer;
}
pub type NonEmptyStringType = String;
#[derive(Debug)]
pub struct OpcodeType {
    pub pri_opcd: Vec<PriopcdType>,
}
impl WithDeserializer for OpcodeType {
    type Deserializer = quick_xml_deserialize::OpcodeTypeDeserializer;
}
#[derive(Debug)]
pub struct OperandType {
    pub nr: Option<OperandTypeNrType>,
    pub group: Option<OperandTypeGroupType>,
    pub type_: Option<String>,
    pub depend: Option<YesNoType>,
    pub address: Option<AddressType>,
    pub displayed: Option<YesNoType>,
    pub a: Option<String>,
    pub t: Option<String>,
}
impl WithDeserializer for OperandType {
    type Deserializer = quick_xml_deserialize::OperandTypeDeserializer;
}
#[derive(Debug)]
pub struct PriopcdType {
    pub value: String,
    pub proc_start: Option<i32>,
    pub entry: Vec<EntryType>,
}
impl WithDeserializer for PriopcdType {
    type Deserializer = quick_xml_deserialize::PriopcdTypeDeserializer;
}
#[derive(Debug)]
pub struct RingnotesType {
    pub ring_note: Vec<GennotesTypeGenNoteElementType>,
}
impl WithDeserializer for RingnotesType {
    type Deserializer = quick_xml_deserialize::RingnotesTypeDeserializer;
}
#[derive(Debug)]
pub struct SkipmeType;
impl WithDeserializer for SkipmeType {
    type Deserializer = quick_xml_deserialize::SkipmeTypeDeserializer;
}
#[derive(Debug)]
pub struct SyntaxType {
    pub mod_: Option<String>,
    pub content: Vec<SyntaxTypeContent>,
}
#[derive(Debug)]
pub enum SyntaxTypeContent {
    Mnem(SyntaxTypeMnemElementType),
    Src(OperandType),
    Dst(OperandType),
}
impl WithDeserializer for SyntaxType {
    type Deserializer = quick_xml_deserialize::SyntaxTypeDeserializer;
}
impl WithDeserializer for SyntaxTypeContent {
    type Deserializer = quick_xml_deserialize::SyntaxTypeContentDeserializer;
}
pub type X86Reference = X86ReferenceElementType;
#[derive(Debug)]
pub struct X86ReferenceElementType {
    pub version: Option<String>,
    pub one_byte: OpcodeType,
    pub two_byte: OpcodeType,
    pub gen_notes: GennotesType,
    pub ring_notes: RingnotesType,
}
impl WithDeserializer for X86ReferenceElementType {
    type Deserializer = quick_xml_deserialize::X86ReferenceElementTypeDeserializer;
}
#[derive(Debug)]
pub enum YesNoType {
    Yes,
    No,
}
impl DeserializeBytes for YesNoType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"yes" => Ok(Self::Yes),
            b"no" => Ok(Self::No),
            x => Err(reader
                .map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum EntryTypeModType {
    Mem,
    Nomem,
}
impl DeserializeBytes for EntryTypeModType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"mem" => Ok(Self::Mem),
            b"nomem" => Ok(Self::Nomem),
            x => Err(reader
                .map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum EntryTypeDocType {
    M,
    U,
}
impl DeserializeBytes for EntryTypeDocType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"m" => Ok(Self::M),
            b"u" => Ok(Self::U),
            x => Err(reader
                .map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum EntryTypeRingType {
    _0,
    F,
}
impl DeserializeBytes for EntryTypeRingType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"0" => Ok(Self::_0),
            b"f" => Ok(Self::F),
            x => Err(reader
                .map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum EntryTypeRingRefType {
    Cr4Tsd,
    Cr4Pce,
    RflagsIopl,
}
impl DeserializeBytes for EntryTypeRingRefType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"cr4_tsd" => Ok(Self::Cr4Tsd),
            b"cr4_pce" => Ok(Self::Cr4Pce),
            b"rflags_iopl" => Ok(Self::RflagsIopl),
            x => Err(reader
                .map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum EntryTypeModeType {
    E,
    P,
    S,
}
impl DeserializeBytes for EntryTypeModeType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"e" => Ok(Self::E),
            b"p" => Ok(Self::P),
            b"s" => Ok(Self::S),
            x => Err(reader
                .map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub enum EntryTypeAttrType {
    Serial,
    Invd,
    Acc,
    Delaysint,
    Undef,
    Null,
    Nop,
    DelaysintCond,
}
impl DeserializeBytes for EntryTypeAttrType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"serial" => Ok(Self::Serial),
            b"invd" => Ok(Self::Invd),
            b"acc" => Ok(Self::Acc),
            b"delaysint" => Ok(Self::Delaysint),
            b"undef" => Ok(Self::Undef),
            b"null" => Ok(Self::Null),
            b"nop" => Ok(Self::Nop),
            b"delaysint_cond" => Ok(Self::DelaysintCond),
            x => Err(reader
                .map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct EntryTypeSecOpcdElementType {
    pub escape: Option<YesNoType>,
    pub content: String,
}
impl WithDeserializer for EntryTypeSecOpcdElementType {
    type Deserializer = quick_xml_deserialize::EntryTypeSecOpcdElementTypeDeserializer;
}
#[derive(Debug)]
pub struct EntryTypeProcStartElementType {
    pub post: Option<YesNoType>,
    pub lat_step: Option<YesNoType>,
    pub content: i32,
}
impl WithDeserializer for EntryTypeProcStartElementType {
    type Deserializer = quick_xml_deserialize::EntryTypeProcStartElementTypeDeserializer;
}
#[derive(Debug)]
pub enum EntryTypeInstrExtElementType {
    Sse1,
    Mmx,
    Sse2,
    Sse3,
    Ssse3,
    Vmx,
    Smx,
    Sse41,
    Sse42,
    Lzcnt,
    Bmi1,
}
impl DeserializeBytes for EntryTypeInstrExtElementType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"sse1" => Ok(Self::Sse1),
            b"mmx" => Ok(Self::Mmx),
            b"sse2" => Ok(Self::Sse2),
            b"sse3" => Ok(Self::Sse3),
            b"ssse3" => Ok(Self::Ssse3),
            b"vmx" => Ok(Self::Vmx),
            b"smx" => Ok(Self::Smx),
            b"sse41" => Ok(Self::Sse41),
            b"sse42" => Ok(Self::Sse42),
            b"lzcnt" => Ok(Self::Lzcnt),
            b"bmi1" => Ok(Self::Bmi1),
            x => Err(reader
                .map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct EntryTypeModifFElementType {
    pub cond: Option<YesNoType>,
    pub content: String,
}
impl WithDeserializer for EntryTypeModifFElementType {
    type Deserializer = quick_xml_deserialize::EntryTypeModifFElementTypeDeserializer;
}
#[derive(Debug)]
pub struct EntryTypeDefFElementType {
    pub cond: Option<String>,
    pub content: String,
}
impl WithDeserializer for EntryTypeDefFElementType {
    type Deserializer = quick_xml_deserialize::EntryTypeDefFElementTypeDeserializer;
}
#[derive(Debug)]
pub struct GennotesTypeGenNoteElementType {
    pub id: String,
}
impl WithDeserializer for GennotesTypeGenNoteElementType {
    type Deserializer =
        quick_xml_deserialize::GennotesTypeGenNoteElementTypeDeserializer;
}
#[derive(Debug)]
pub enum OperandTypeNrType {
    I32(i32),
    C0000102,
    C0000103,
    C0000084,
    C0000082,
    C0000081,
    _8B,
}
impl DeserializeBytes for OperandTypeNrType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"C0000102" => Ok(Self::C0000102),
            b"C0000103" => Ok(Self::C0000103),
            b"C0000084" => Ok(Self::C0000084),
            b"C0000082" => Ok(Self::C0000082),
            b"C0000081" => Ok(Self::C0000081),
            b"8B" => Ok(Self::_8B),
            x => Ok(Self::I32(i32::deserialize_bytes(reader, x)?)),
        }
    }
}
#[derive(Debug)]
pub enum OperandTypeGroupType {
    Gen,
    X87Fpu,
    Seg,
    Systabp,
    Ctrl,
    Msr,
    Xmm,
    Mmx,
    Debug,
    Xcr,
}
impl DeserializeBytes for OperandTypeGroupType {
    fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
    where
        R: DeserializeReader,
    {
        match bytes {
            b"gen" => Ok(Self::Gen),
            b"x87fpu" => Ok(Self::X87Fpu),
            b"seg" => Ok(Self::Seg),
            b"systabp" => Ok(Self::Systabp),
            b"ctrl" => Ok(Self::Ctrl),
            b"msr" => Ok(Self::Msr),
            b"xmm" => Ok(Self::Xmm),
            b"mmx" => Ok(Self::Mmx),
            b"debug" => Ok(Self::Debug),
            b"xcr" => Ok(Self::Xcr),
            x => Err(reader
                .map_error(ErrorKind::UnknownOrInvalidValue(RawByteStr::from_slice(x)))),
        }
    }
}
#[derive(Debug)]
pub struct SyntaxTypeMnemElementType {
    pub sug: Option<YesNoType>,
    pub content: String,
}
impl WithDeserializer for SyntaxTypeMnemElementType {
    type Deserializer = quick_xml_deserialize::SyntaxTypeMnemElementTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, ContentDeserializer, DeserializeReader,
        Deserializer, DeserializerArtifact, DeserializerEvent, DeserializerOutput,
        DeserializerResult, ElementHandlerOutput, Error, ErrorKind, Event, RawByteStr,
        WithDeserializer,
    };
    #[derive(Debug)]
    pub struct EntryTypeDeserializer {
        doc_part_alias_ref: Option<String>,
        part_alias: Option<String>,
        mod_: Option<super::EntryTypeModType>,
        fpush: Option<super::YesNoType>,
        fpop: Option<String>,
        mem_format: Option<String>,
        doc_64_ref: Option<String>,
        alias: Option<String>,
        tttn: Option<String>,
        sign_ext: Option<String>,
        particular: Option<super::YesNoType>,
        doc: Option<super::EntryTypeDocType>,
        doc_ref: Option<String>,
        is_doc: Option<super::YesNoType>,
        is_undoc: Option<super::YesNoType>,
        ring: Option<super::EntryTypeRingType>,
        ring_ref: Option<super::EntryTypeRingRefType>,
        ref_: Option<String>,
        mode: Option<super::EntryTypeModeType>,
        doc_1632_ref: Option<String>,
        r: Option<super::YesNoType>,
        attr: Option<super::EntryTypeAttrType>,
        lock: Option<super::YesNoType>,
        direction: Option<super::OneOrZeroType>,
        op_size: Option<super::OneOrZeroType>,
        pref: Option<String>,
        opcd_ext: Option<i32>,
        sec_opcd: Option<super::EntryTypeSecOpcdElementType>,
        proc_start: Option<super::EntryTypeProcStartElementType>,
        proc_end: Option<i32>,
        syntax: Vec<super::SyntaxType>,
        instr_ext: Option<super::EntryTypeInstrExtElementType>,
        grp_1: Option<String>,
        grp_2: Vec<String>,
        grp_3: Vec<String>,
        test_f: Option<String>,
        modif_f: Option<super::EntryTypeModifFElementType>,
        def_f: Option<super::EntryTypeDefFElementType>,
        undef_f: Option<String>,
        f_vals: Option<String>,
        modif_f_fpu: Option<String>,
        def_f_fpu: Option<String>,
        undef_f_fpu: Option<String>,
        f_vals_fpu: Option<String>,
        note: Option<super::SkipmeType>,
        state: Box<EntryTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EntryTypeDeserializerState {
        Init__,
        Pref(Option<<String as WithDeserializer>::Deserializer>),
        OpcdExt(Option<<i32 as WithDeserializer>::Deserializer>),
        SecOpcd(
            Option<
                <super::EntryTypeSecOpcdElementType as WithDeserializer>::Deserializer,
            >,
        ),
        ProcStart(
            Option<
                <super::EntryTypeProcStartElementType as WithDeserializer>::Deserializer,
            >,
        ),
        ProcEnd(Option<<i32 as WithDeserializer>::Deserializer>),
        Syntax(Option<<super::SyntaxType as WithDeserializer>::Deserializer>),
        InstrExt(
            Option<
                <super::EntryTypeInstrExtElementType as WithDeserializer>::Deserializer,
            >,
        ),
        Grp1(Option<<String as WithDeserializer>::Deserializer>),
        Grp2(Option<<String as WithDeserializer>::Deserializer>),
        Grp3(Option<<String as WithDeserializer>::Deserializer>),
        TestF(Option<<String as WithDeserializer>::Deserializer>),
        ModifF(
            Option<
                <super::EntryTypeModifFElementType as WithDeserializer>::Deserializer,
            >,
        ),
        DefF(
            Option<<super::EntryTypeDefFElementType as WithDeserializer>::Deserializer>,
        ),
        UndefF(Option<<String as WithDeserializer>::Deserializer>),
        FVals(Option<<String as WithDeserializer>::Deserializer>),
        ModifFFpu(Option<<String as WithDeserializer>::Deserializer>),
        DefFFpu(Option<<String as WithDeserializer>::Deserializer>),
        UndefFFpu(Option<<String as WithDeserializer>::Deserializer>),
        FValsFpu(Option<<String as WithDeserializer>::Deserializer>),
        Note(Option<<super::SkipmeType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl EntryTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut doc_part_alias_ref: Option<String> = None;
            let mut part_alias: Option<String> = None;
            let mut mod_: Option<super::EntryTypeModType> = None;
            let mut fpush: Option<super::YesNoType> = None;
            let mut fpop: Option<String> = None;
            let mut mem_format: Option<String> = None;
            let mut doc_64_ref: Option<String> = None;
            let mut alias: Option<String> = None;
            let mut tttn: Option<String> = None;
            let mut sign_ext: Option<String> = None;
            let mut particular: Option<super::YesNoType> = None;
            let mut doc: Option<super::EntryTypeDocType> = None;
            let mut doc_ref: Option<String> = None;
            let mut is_doc: Option<super::YesNoType> = None;
            let mut is_undoc: Option<super::YesNoType> = None;
            let mut ring: Option<super::EntryTypeRingType> = None;
            let mut ring_ref: Option<super::EntryTypeRingRefType> = None;
            let mut ref_: Option<String> = None;
            let mut mode: Option<super::EntryTypeModeType> = None;
            let mut doc_1632_ref: Option<String> = None;
            let mut r: Option<super::YesNoType> = None;
            let mut attr: Option<super::EntryTypeAttrType> = None;
            let mut lock: Option<super::YesNoType> = None;
            let mut direction: Option<super::OneOrZeroType> = None;
            let mut op_size: Option<super::OneOrZeroType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"doc_part_alias_ref" {
                    reader.read_attrib(
                        &mut doc_part_alias_ref,
                        b"doc_part_alias_ref",
                        &attrib.value,
                    )?;
                } else if attrib.key.local_name().as_ref() == b"part_alias" {
                    reader.read_attrib(&mut part_alias, b"part_alias", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"mod" {
                    reader.read_attrib(&mut mod_, b"mod", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"fpush" {
                    reader.read_attrib(&mut fpush, b"fpush", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"fpop" {
                    reader.read_attrib(&mut fpop, b"fpop", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"mem_format" {
                    reader.read_attrib(&mut mem_format, b"mem_format", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"doc64_ref" {
                    reader.read_attrib(&mut doc_64_ref, b"doc64_ref", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"alias" {
                    reader.read_attrib(&mut alias, b"alias", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"tttn" {
                    reader.read_attrib(&mut tttn, b"tttn", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"sign-ext" {
                    reader.read_attrib(&mut sign_ext, b"sign-ext", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"particular" {
                    reader.read_attrib(&mut particular, b"particular", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"doc" {
                    reader.read_attrib(&mut doc, b"doc", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"doc_ref" {
                    reader.read_attrib(&mut doc_ref, b"doc_ref", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"is_doc" {
                    reader.read_attrib(&mut is_doc, b"is_doc", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"is_undoc" {
                    reader.read_attrib(&mut is_undoc, b"is_undoc", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"ring" {
                    reader.read_attrib(&mut ring, b"ring", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"ring_ref" {
                    reader.read_attrib(&mut ring_ref, b"ring_ref", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"ref" {
                    reader.read_attrib(&mut ref_, b"ref", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"mode" {
                    reader.read_attrib(&mut mode, b"mode", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"doc1632_ref" {
                    reader.read_attrib(
                        &mut doc_1632_ref,
                        b"doc1632_ref",
                        &attrib.value,
                    )?;
                } else if attrib.key.local_name().as_ref() == b"r" {
                    reader.read_attrib(&mut r, b"r", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"attr" {
                    reader.read_attrib(&mut attr, b"attr", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"lock" {
                    reader.read_attrib(&mut lock, b"lock", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"direction" {
                    reader.read_attrib(&mut direction, b"direction", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"op_size" {
                    reader.read_attrib(&mut op_size, b"op_size", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                doc_part_alias_ref: doc_part_alias_ref,
                part_alias: part_alias,
                mod_: mod_,
                fpush: fpush,
                fpop: fpop,
                mem_format: mem_format,
                doc_64_ref: doc_64_ref,
                alias: alias,
                tttn: tttn,
                sign_ext: sign_ext,
                particular: particular,
                doc: doc,
                doc_ref: doc_ref,
                is_doc: is_doc,
                is_undoc: is_undoc,
                ring: ring,
                ring_ref: ring_ref,
                ref_: ref_,
                mode: mode,
                doc_1632_ref: doc_1632_ref,
                r: r,
                attr: attr,
                lock: lock,
                direction: direction,
                op_size: op_size,
                pref: None,
                opcd_ext: None,
                sec_opcd: None,
                proc_start: None,
                proc_end: None,
                syntax: Vec::new(),
                instr_ext: None,
                grp_1: None,
                grp_2: Vec::new(),
                grp_3: Vec::new(),
                test_f: None,
                modif_f: None,
                def_f: None,
                undef_f: None,
                f_vals: None,
                modif_f_fpu: None,
                def_f_fpu: None,
                undef_f_fpu: None,
                f_vals_fpu: None,
                note: None,
                state: Box::new(EntryTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: EntryTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use EntryTypeDeserializerState as S;
            match state {
                S::Pref(Some(deserializer)) => {
                    self.store_pref(deserializer.finish(reader)?)?
                }
                S::OpcdExt(Some(deserializer)) => {
                    self.store_opcd_ext(deserializer.finish(reader)?)?
                }
                S::SecOpcd(Some(deserializer)) => {
                    self.store_sec_opcd(deserializer.finish(reader)?)?
                }
                S::ProcStart(Some(deserializer)) => {
                    self.store_proc_start(deserializer.finish(reader)?)?
                }
                S::ProcEnd(Some(deserializer)) => {
                    self.store_proc_end(deserializer.finish(reader)?)?
                }
                S::Syntax(Some(deserializer)) => {
                    self.store_syntax(deserializer.finish(reader)?)?
                }
                S::InstrExt(Some(deserializer)) => {
                    self.store_instr_ext(deserializer.finish(reader)?)?
                }
                S::Grp1(Some(deserializer)) => {
                    self.store_grp_1(deserializer.finish(reader)?)?
                }
                S::Grp2(Some(deserializer)) => {
                    self.store_grp_2(deserializer.finish(reader)?)?
                }
                S::Grp3(Some(deserializer)) => {
                    self.store_grp_3(deserializer.finish(reader)?)?
                }
                S::TestF(Some(deserializer)) => {
                    self.store_test_f(deserializer.finish(reader)?)?
                }
                S::ModifF(Some(deserializer)) => {
                    self.store_modif_f(deserializer.finish(reader)?)?
                }
                S::DefF(Some(deserializer)) => {
                    self.store_def_f(deserializer.finish(reader)?)?
                }
                S::UndefF(Some(deserializer)) => {
                    self.store_undef_f(deserializer.finish(reader)?)?
                }
                S::FVals(Some(deserializer)) => {
                    self.store_f_vals(deserializer.finish(reader)?)?
                }
                S::ModifFFpu(Some(deserializer)) => {
                    self.store_modif_f_fpu(deserializer.finish(reader)?)?
                }
                S::DefFFpu(Some(deserializer)) => {
                    self.store_def_f_fpu(deserializer.finish(reader)?)?
                }
                S::UndefFFpu(Some(deserializer)) => {
                    self.store_undef_f_fpu(deserializer.finish(reader)?)?
                }
                S::FValsFpu(Some(deserializer)) => {
                    self.store_f_vals_fpu(deserializer.finish(reader)?)?
                }
                S::Note(Some(deserializer)) => {
                    self.store_note(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_pref(&mut self, value: String) -> Result<(), Error> {
            if self.pref.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"pref")))?;
            }
            self.pref = Some(value);
            Ok(())
        }
        fn store_opcd_ext(&mut self, value: i32) -> Result<(), Error> {
            if self.opcd_ext.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"opcd_ext",
                )))?;
            }
            self.opcd_ext = Some(value);
            Ok(())
        }
        fn store_sec_opcd(
            &mut self,
            value: super::EntryTypeSecOpcdElementType,
        ) -> Result<(), Error> {
            if self.sec_opcd.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"sec_opcd",
                )))?;
            }
            self.sec_opcd = Some(value);
            Ok(())
        }
        fn store_proc_start(
            &mut self,
            value: super::EntryTypeProcStartElementType,
        ) -> Result<(), Error> {
            if self.proc_start.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"proc_start",
                )))?;
            }
            self.proc_start = Some(value);
            Ok(())
        }
        fn store_proc_end(&mut self, value: i32) -> Result<(), Error> {
            if self.proc_end.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"proc_end",
                )))?;
            }
            self.proc_end = Some(value);
            Ok(())
        }
        fn store_syntax(&mut self, value: super::SyntaxType) -> Result<(), Error> {
            self.syntax.push(value);
            Ok(())
        }
        fn store_instr_ext(
            &mut self,
            value: super::EntryTypeInstrExtElementType,
        ) -> Result<(), Error> {
            if self.instr_ext.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"instr_ext",
                )))?;
            }
            self.instr_ext = Some(value);
            Ok(())
        }
        fn store_grp_1(&mut self, value: String) -> Result<(), Error> {
            if self.grp_1.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"grp1")))?;
            }
            self.grp_1 = Some(value);
            Ok(())
        }
        fn store_grp_2(&mut self, value: String) -> Result<(), Error> {
            self.grp_2.push(value);
            Ok(())
        }
        fn store_grp_3(&mut self, value: String) -> Result<(), Error> {
            self.grp_3.push(value);
            Ok(())
        }
        fn store_test_f(&mut self, value: String) -> Result<(), Error> {
            if self.test_f.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"test_f",
                )))?;
            }
            self.test_f = Some(value);
            Ok(())
        }
        fn store_modif_f(
            &mut self,
            value: super::EntryTypeModifFElementType,
        ) -> Result<(), Error> {
            if self.modif_f.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"modif_f",
                )))?;
            }
            self.modif_f = Some(value);
            Ok(())
        }
        fn store_def_f(
            &mut self,
            value: super::EntryTypeDefFElementType,
        ) -> Result<(), Error> {
            if self.def_f.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"def_f",
                )))?;
            }
            self.def_f = Some(value);
            Ok(())
        }
        fn store_undef_f(&mut self, value: String) -> Result<(), Error> {
            if self.undef_f.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"undef_f",
                )))?;
            }
            self.undef_f = Some(value);
            Ok(())
        }
        fn store_f_vals(&mut self, value: String) -> Result<(), Error> {
            if self.f_vals.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"f_vals",
                )))?;
            }
            self.f_vals = Some(value);
            Ok(())
        }
        fn store_modif_f_fpu(&mut self, value: String) -> Result<(), Error> {
            if self.modif_f_fpu.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"modif_f_fpu",
                )))?;
            }
            self.modif_f_fpu = Some(value);
            Ok(())
        }
        fn store_def_f_fpu(&mut self, value: String) -> Result<(), Error> {
            if self.def_f_fpu.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"def_f_fpu",
                )))?;
            }
            self.def_f_fpu = Some(value);
            Ok(())
        }
        fn store_undef_f_fpu(&mut self, value: String) -> Result<(), Error> {
            if self.undef_f_fpu.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"undef_f_fpu",
                )))?;
            }
            self.undef_f_fpu = Some(value);
            Ok(())
        }
        fn store_f_vals_fpu(&mut self, value: String) -> Result<(), Error> {
            if self.f_vals_fpu.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"f_vals_fpu",
                )))?;
            }
            self.f_vals_fpu = Some(value);
            Ok(())
        }
        fn store_note(&mut self, value: super::SkipmeType) -> Result<(), Error> {
            if self.note.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"note")))?;
            }
            self.note = Some(value);
            Ok(())
        }
        fn handle_pref<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::Pref(None));
                *self.state = EntryTypeDeserializerState::OpcdExt(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pref(data)?;
                    *self.state = EntryTypeDeserializerState::OpcdExt(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::Pref(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::OpcdExt(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::Pref(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_opcd_ext<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::OpcdExt(None));
                *self.state = EntryTypeDeserializerState::SecOpcd(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_opcd_ext(data)?;
                    *self.state = EntryTypeDeserializerState::SecOpcd(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::OpcdExt(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::SecOpcd(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::OpcdExt(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_sec_opcd<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::EntryTypeSecOpcdElementType>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::SecOpcd(None));
                *self.state = EntryTypeDeserializerState::ProcStart(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_sec_opcd(data)?;
                    *self.state = EntryTypeDeserializerState::ProcStart(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::SecOpcd(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::ProcStart(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::SecOpcd(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_proc_start<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::EntryTypeProcStartElementType>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::ProcStart(None));
                *self.state = EntryTypeDeserializerState::ProcEnd(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_proc_start(data)?;
                    *self.state = EntryTypeDeserializerState::ProcEnd(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                EntryTypeDeserializerState::ProcStart(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = EntryTypeDeserializerState::ProcEnd(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = EntryTypeDeserializerState::ProcStart(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_proc_end<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::ProcEnd(None));
                *self.state = EntryTypeDeserializerState::Syntax(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_proc_end(data)?;
                    *self.state = EntryTypeDeserializerState::Syntax(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::ProcEnd(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::Syntax(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::ProcEnd(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_syntax<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SyntaxType>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.syntax.len() < 1usize {
                    *self.state = EntryTypeDeserializerState::Syntax(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(EntryTypeDeserializerState::Syntax(None));
                    *self.state = EntryTypeDeserializerState::InstrExt(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_syntax(data)?;
                    *self.state = EntryTypeDeserializerState::Syntax(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::Syntax(
                                Some(deserializer),
                            ));
                            if self.syntax.len().saturating_add(1) < 1usize {
                                *self.state = EntryTypeDeserializerState::Syntax(None);
                            } else {
                                *self.state = EntryTypeDeserializerState::InstrExt(None);
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::Syntax(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_instr_ext<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::EntryTypeInstrExtElementType>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::InstrExt(None));
                *self.state = EntryTypeDeserializerState::Grp1(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_instr_ext(data)?;
                    *self.state = EntryTypeDeserializerState::Grp1(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                EntryTypeDeserializerState::InstrExt(Some(deserializer)),
                            );
                            *self.state = EntryTypeDeserializerState::Grp1(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::InstrExt(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_grp_1<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::Grp1(None));
                *self.state = EntryTypeDeserializerState::Grp2(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_grp_1(data)?;
                    *self.state = EntryTypeDeserializerState::Grp2(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::Grp1(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::Grp2(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::Grp1(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_grp_2<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::Grp2(None));
                *self.state = EntryTypeDeserializerState::Grp3(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_grp_2(data)?;
                    if self.grp_2.len() < 2usize {
                        *self.state = EntryTypeDeserializerState::Grp2(None);
                    } else {
                        *self.state = EntryTypeDeserializerState::Grp3(None);
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::Grp2(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::Grp2(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::Grp2(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_grp_3<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::Grp3(None));
                *self.state = EntryTypeDeserializerState::TestF(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_grp_3(data)?;
                    if self.grp_3.len() < 2usize {
                        *self.state = EntryTypeDeserializerState::Grp3(None);
                    } else {
                        *self.state = EntryTypeDeserializerState::TestF(None);
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::Grp3(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::Grp3(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::Grp3(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_test_f<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::TestF(None));
                *self.state = EntryTypeDeserializerState::ModifF(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_test_f(data)?;
                    *self.state = EntryTypeDeserializerState::ModifF(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::TestF(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::ModifF(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::TestF(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_modif_f<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::EntryTypeModifFElementType>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::ModifF(None));
                *self.state = EntryTypeDeserializerState::DefF(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_modif_f(data)?;
                    *self.state = EntryTypeDeserializerState::DefF(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::ModifF(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::DefF(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::ModifF(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_def_f<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::EntryTypeDefFElementType>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::DefF(None));
                *self.state = EntryTypeDeserializerState::UndefF(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_def_f(data)?;
                    *self.state = EntryTypeDeserializerState::UndefF(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::DefF(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::UndefF(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::DefF(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_undef_f<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::UndefF(None));
                *self.state = EntryTypeDeserializerState::FVals(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_undef_f(data)?;
                    *self.state = EntryTypeDeserializerState::FVals(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::UndefF(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::FVals(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::UndefF(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_f_vals<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::FVals(None));
                *self.state = EntryTypeDeserializerState::ModifFFpu(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_f_vals(data)?;
                    *self.state = EntryTypeDeserializerState::ModifFFpu(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::FVals(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::ModifFFpu(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::FVals(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_modif_f_fpu<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::ModifFFpu(None));
                *self.state = EntryTypeDeserializerState::DefFFpu(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_modif_f_fpu(data)?;
                    *self.state = EntryTypeDeserializerState::DefFFpu(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                EntryTypeDeserializerState::ModifFFpu(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = EntryTypeDeserializerState::DefFFpu(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = EntryTypeDeserializerState::ModifFFpu(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_def_f_fpu<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::DefFFpu(None));
                *self.state = EntryTypeDeserializerState::UndefFFpu(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_def_f_fpu(data)?;
                    *self.state = EntryTypeDeserializerState::UndefFFpu(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::DefFFpu(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::UndefFFpu(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::DefFFpu(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_undef_f_fpu<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::UndefFFpu(None));
                *self.state = EntryTypeDeserializerState::FValsFpu(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_undef_f_fpu(data)?;
                    *self.state = EntryTypeDeserializerState::FValsFpu(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                EntryTypeDeserializerState::UndefFFpu(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = EntryTypeDeserializerState::FValsFpu(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = EntryTypeDeserializerState::UndefFFpu(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_f_vals_fpu<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::FValsFpu(None));
                *self.state = EntryTypeDeserializerState::Note(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_f_vals_fpu(data)?;
                    *self.state = EntryTypeDeserializerState::Note(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                EntryTypeDeserializerState::FValsFpu(Some(deserializer)),
                            );
                            *self.state = EntryTypeDeserializerState::Note(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::FValsFpu(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_note<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SkipmeType>,
            fallback: &mut Option<EntryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(EntryTypeDeserializerState::Note(None));
                *self.state = EntryTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_note(data)?;
                    *self.state = EntryTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(EntryTypeDeserializerState::Note(
                                Some(deserializer),
                            ));
                            *self.state = EntryTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                EntryTypeDeserializerState::Note(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::EntryType> for EntryTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EntryType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EntryType>
        where
            R: DeserializeReader,
        {
            use EntryTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Pref(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pref(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::OpcdExt(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_opcd_ext(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SecOpcd(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_sec_opcd(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ProcStart(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_proc_start(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ProcEnd(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_proc_end(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Syntax(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_syntax(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::InstrExt(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_instr_ext(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Grp1(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_grp_1(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Grp2(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_grp_2(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Grp3(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_grp_3(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TestF(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_test_f(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ModifF(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_modif_f(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DefF(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_def_f(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::UndefF(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_undef_f(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::FVals(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_f_vals(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ModifFFpu(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_modif_f_fpu(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DefFFpu(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_def_f_fpu(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::UndefFFpu(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_undef_f_fpu(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::FValsFpu(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_f_vals_fpu(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Note(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_note(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = EntryTypeDeserializerState::Pref(None);
                        event
                    }
                    (S::Pref(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"pref") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_pref(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::OpcdExt(None);
                            event
                        }
                    }
                    (S::OpcdExt(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"opcd_ext") {
                            let output = <i32 as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_opcd_ext(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SecOpcd(None);
                            event
                        }
                    }
                    (S::SecOpcd(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"sec_opcd") {
                            let output = < super :: EntryTypeSecOpcdElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_sec_opcd(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ProcStart(None);
                            event
                        }
                    }
                    (
                        S::ProcStart(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(&event, None, b"proc_start") {
                            let output = < super :: EntryTypeProcStartElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_proc_start(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ProcEnd(None);
                            event
                        }
                    }
                    (S::ProcEnd(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"proc_end") {
                            let output = <i32 as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_proc_end(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Syntax(None);
                            event
                        }
                    }
                    (S::Syntax(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"syntax") {
                            let output = < super :: SyntaxType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_syntax(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::InstrExt(None);
                            event
                        }
                    }
                    (S::InstrExt(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"instr_ext") {
                            let output = < super :: EntryTypeInstrExtElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_instr_ext(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Grp1(None);
                            event
                        }
                    }
                    (S::Grp1(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"grp1") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_grp_1(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Grp2(None);
                            event
                        }
                    }
                    (S::Grp2(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"grp2") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_grp_2(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Grp3(None);
                            event
                        }
                    }
                    (S::Grp3(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"grp3") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_grp_3(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::TestF(None);
                            event
                        }
                    }
                    (S::TestF(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"test_f") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_test_f(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ModifF(None);
                            event
                        }
                    }
                    (S::ModifF(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"modif_f") {
                            let output = < super :: EntryTypeModifFElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_modif_f(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::DefF(None);
                            event
                        }
                    }
                    (S::DefF(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"def_f") {
                            let output = < super :: EntryTypeDefFElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_def_f(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::UndefF(None);
                            event
                        }
                    }
                    (S::UndefF(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"undef_f") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_undef_f(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::FVals(None);
                            event
                        }
                    }
                    (S::FVals(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"f_vals") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_f_vals(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ModifFFpu(None);
                            event
                        }
                    }
                    (
                        S::ModifFFpu(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(&event, None, b"modif_f_fpu") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_modif_f_fpu(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::DefFFpu(None);
                            event
                        }
                    }
                    (S::DefFFpu(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"def_f_fpu") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_def_f_fpu(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::UndefFFpu(None);
                            event
                        }
                    }
                    (
                        S::UndefFFpu(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(&event, None, b"undef_f_fpu") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_undef_f_fpu(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::FValsFpu(None);
                            event
                        }
                    }
                    (S::FValsFpu(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"f_vals_fpu") {
                            let output =
                                <String as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_f_vals_fpu(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Note(None);
                            event
                        }
                    }
                    (S::Note(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"note") {
                            let output = < super :: SkipmeType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_note(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            allow_any_element = true;
                            fallback.get_or_insert(S::Note(None));
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::EntryType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, EntryTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::EntryType {
                doc_part_alias_ref: self.doc_part_alias_ref,
                part_alias: self.part_alias,
                mod_: self.mod_,
                fpush: self.fpush,
                fpop: self.fpop,
                mem_format: self.mem_format,
                doc_64_ref: self.doc_64_ref,
                alias: self.alias,
                tttn: self.tttn,
                sign_ext: self.sign_ext,
                particular: self.particular,
                doc: self.doc,
                doc_ref: self.doc_ref,
                is_doc: self.is_doc,
                is_undoc: self.is_undoc,
                ring: self.ring,
                ring_ref: self.ring_ref,
                ref_: self.ref_,
                mode: self.mode,
                doc_1632_ref: self.doc_1632_ref,
                r: self.r,
                attr: self.attr,
                lock: self.lock,
                direction: self.direction,
                op_size: self.op_size,
                pref: self.pref,
                opcd_ext: self.opcd_ext,
                sec_opcd: self.sec_opcd,
                proc_start: self.proc_start,
                proc_end: self.proc_end,
                syntax: self.syntax,
                instr_ext: self.instr_ext,
                grp_1: self.grp_1,
                grp_2: self.grp_2,
                grp_3: self.grp_3,
                test_f: self.test_f,
                modif_f: self.modif_f,
                def_f: self.def_f,
                undef_f: self.undef_f,
                f_vals: self.f_vals,
                modif_f_fpu: self.modif_f_fpu,
                def_f_fpu: self.def_f_fpu,
                undef_f_fpu: self.undef_f_fpu,
                f_vals_fpu: self.f_vals_fpu,
                note: self.note,
            })
        }
    }
    #[derive(Debug)]
    pub struct GennotesTypeDeserializer {
        gen_note: Vec<super::GennotesTypeGenNoteElementType>,
        state: Box<GennotesTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum GennotesTypeDeserializerState {
        Init__ , GenNote (Option << super :: GennotesTypeGenNoteElementType as WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl GennotesTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                gen_note: Vec::new(),
                state: Box::new(GennotesTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: GennotesTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use GennotesTypeDeserializerState as S;
            match state {
                S::GenNote(Some(deserializer)) => {
                    self.store_gen_note(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_gen_note(
            &mut self,
            value: super::GennotesTypeGenNoteElementType,
        ) -> Result<(), Error> {
            self.gen_note.push(value);
            Ok(())
        }
        fn handle_gen_note<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::GennotesTypeGenNoteElementType>,
            fallback: &mut Option<GennotesTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.gen_note.len() < 1usize {
                    *self.state = GennotesTypeDeserializerState::GenNote(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(GennotesTypeDeserializerState::GenNote(None));
                    *self.state = GennotesTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_gen_note(data)?;
                    *self.state = GennotesTypeDeserializerState::GenNote(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                GennotesTypeDeserializerState::GenNote(Some(
                                    deserializer,
                                )),
                            );
                            if self.gen_note.len().saturating_add(1) < 1usize {
                                *self.state =
                                    GennotesTypeDeserializerState::GenNote(None);
                            } else {
                                *self.state = GennotesTypeDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = GennotesTypeDeserializerState::GenNote(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::GennotesType> for GennotesTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GennotesType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GennotesType>
        where
            R: DeserializeReader,
        {
            use GennotesTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::GenNote(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_gen_note(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = GennotesTypeDeserializerState::GenNote(None);
                        event
                    }
                    (S::GenNote(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"gen_note") {
                            let output = < super :: GennotesTypeGenNoteElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_gen_note(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::GennotesType, Error>
        where
            R: DeserializeReader,
        {
            let state =
                replace(&mut *self.state, GennotesTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::GennotesType {
                gen_note: self.gen_note,
            })
        }
    }
    #[derive(Debug)]
    pub struct OpcodeTypeDeserializer {
        pri_opcd: Vec<super::PriopcdType>,
        state: Box<OpcodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum OpcodeTypeDeserializerState {
        Init__,
        PriOpcd(Option<<super::PriopcdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl OpcodeTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            Ok(Self {
                pri_opcd: Vec::new(),
                state: Box::new(OpcodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: OpcodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use OpcodeTypeDeserializerState as S;
            match state {
                S::PriOpcd(Some(deserializer)) => {
                    self.store_pri_opcd(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_pri_opcd(&mut self, value: super::PriopcdType) -> Result<(), Error> {
            self.pri_opcd.push(value);
            Ok(())
        }
        fn handle_pri_opcd<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PriopcdType>,
            fallback: &mut Option<OpcodeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.pri_opcd.len() < 1usize {
                    *self.state = OpcodeTypeDeserializerState::PriOpcd(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(OpcodeTypeDeserializerState::PriOpcd(None));
                    *self.state = OpcodeTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_pri_opcd(data)?;
                    *self.state = OpcodeTypeDeserializerState::PriOpcd(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                OpcodeTypeDeserializerState::PriOpcd(Some(deserializer)),
                            );
                            if self.pri_opcd.len().saturating_add(1) < 1usize {
                                *self.state = OpcodeTypeDeserializerState::PriOpcd(None);
                            } else {
                                *self.state = OpcodeTypeDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                OpcodeTypeDeserializerState::PriOpcd(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::OpcodeType> for OpcodeTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpcodeType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OpcodeType>
        where
            R: DeserializeReader,
        {
            use OpcodeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::PriOpcd(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_pri_opcd(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = OpcodeTypeDeserializerState::PriOpcd(None);
                        event
                    }
                    (S::PriOpcd(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"pri_opcd") {
                            let output = < super :: PriopcdType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_pri_opcd(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::OpcodeType, Error>
        where
            R: DeserializeReader,
        {
            let state =
                replace(&mut *self.state, OpcodeTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::OpcodeType {
                pri_opcd: self.pri_opcd,
            })
        }
    }
    #[derive(Debug)]
    pub struct OperandTypeDeserializer {
        nr: Option<super::OperandTypeNrType>,
        group: Option<super::OperandTypeGroupType>,
        type_: Option<String>,
        depend: Option<super::YesNoType>,
        address: Option<super::AddressType>,
        displayed: Option<super::YesNoType>,
        a: Option<String>,
        t: Option<String>,
        state: Box<OperandTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum OperandTypeDeserializerState {
        Init__,
        Next__,
        A(<String as WithDeserializer>::Deserializer),
        T(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl OperandTypeDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<OperandTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self.state = fallback
                    .take()
                    .unwrap_or(OperandTypeDeserializerState::Init__);
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if x.name().local_name().as_ref() == b"a" {
                let output =
                    <String as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_a(reader, output, &mut *fallback);
            }
            if x.name().local_name().as_ref() == b"t" {
                let output =
                    <String as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_t(reader, output, &mut *fallback);
            }
            *self.state = fallback
                .take()
                .unwrap_or(OperandTypeDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut nr: Option<super::OperandTypeNrType> = None;
            let mut group: Option<super::OperandTypeGroupType> = None;
            let mut type_: Option<String> = None;
            let mut depend: Option<super::YesNoType> = None;
            let mut address: Option<super::AddressType> = None;
            let mut displayed: Option<super::YesNoType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"nr" {
                    reader.read_attrib(&mut nr, b"nr", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"group" {
                    reader.read_attrib(&mut group, b"group", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"type" {
                    reader.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"depend" {
                    reader.read_attrib(&mut depend, b"depend", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"address" {
                    reader.read_attrib(&mut address, b"address", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"displayed" {
                    reader.read_attrib(&mut displayed, b"displayed", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                nr: nr,
                group: group,
                type_: type_,
                depend: depend,
                address: address,
                displayed: displayed,
                a: None,
                t: None,
                state: Box::new(OperandTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: OperandTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use OperandTypeDeserializerState as S;
            match state {
                S::A(deserializer) => self.store_a(deserializer.finish(reader)?)?,
                S::T(deserializer) => self.store_t(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_a(&mut self, value: String) -> Result<(), Error> {
            if self.a.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"a")))?;
            }
            self.a = Some(value);
            Ok(())
        }
        fn store_t(&mut self, value: String) -> Result<(), Error> {
            if self.t.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"t")))?;
            }
            self.t = Some(value);
            Ok(())
        }
        fn handle_a<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<OperandTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                *self.state = match ret {
                    ElementHandlerOutput::Continue { .. } => {
                        OperandTypeDeserializerState::Next__
                    }
                    ElementHandlerOutput::Break { .. } => fallback
                        .take()
                        .unwrap_or(OperandTypeDeserializerState::Next__),
                };
                return Ok(ret);
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_a(data)?;
                    *self.state = OperandTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(OperandTypeDeserializerState::A(
                                deserializer,
                            ));
                            *self.state = OperandTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = OperandTypeDeserializerState::A(deserializer);
                        }
                    }
                    ret
                }
            })
        }
        fn handle_t<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<OperandTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                let ret = ElementHandlerOutput::from_event(event, allow_any);
                *self.state = match ret {
                    ElementHandlerOutput::Continue { .. } => {
                        OperandTypeDeserializerState::Next__
                    }
                    ElementHandlerOutput::Break { .. } => fallback
                        .take()
                        .unwrap_or(OperandTypeDeserializerState::Next__),
                };
                return Ok(ret);
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_t(data)?;
                    *self.state = OperandTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(OperandTypeDeserializerState::T(
                                deserializer,
                            ));
                            *self.state = OperandTypeDeserializerState::Next__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = OperandTypeDeserializerState::T(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::OperandType> for OperandTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OperandType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::OperandType>
        where
            R: DeserializeReader,
        {
            use OperandTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::A(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_a(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::T(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_t(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, .. } => event,
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::OperandType, Error>
        where
            R: DeserializeReader,
        {
            let state =
                replace(&mut *self.state, OperandTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::OperandType {
                nr: self.nr,
                group: self.group,
                type_: self.type_,
                depend: self.depend,
                address: self.address,
                displayed: self.displayed,
                a: self.a,
                t: self.t,
            })
        }
    }
    #[derive(Debug)]
    pub struct PriopcdTypeDeserializer {
        value: String,
        proc_start: Option<i32>,
        entry: Vec<super::EntryType>,
        state: Box<PriopcdTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PriopcdTypeDeserializerState {
        Init__,
        ProcStart(Option<<i32 as WithDeserializer>::Deserializer>),
        Entry(Option<<super::EntryType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl PriopcdTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut value: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"value" {
                    reader.read_attrib(&mut value, b"value", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                value: value.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("value".into()))
                })?,
                proc_start: None,
                entry: Vec::new(),
                state: Box::new(PriopcdTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PriopcdTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use PriopcdTypeDeserializerState as S;
            match state {
                S::ProcStart(Some(deserializer)) => {
                    self.store_proc_start(deserializer.finish(reader)?)?
                }
                S::Entry(Some(deserializer)) => {
                    self.store_entry(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_proc_start(&mut self, value: i32) -> Result<(), Error> {
            if self.proc_start.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"proc_start",
                )))?;
            }
            self.proc_start = Some(value);
            Ok(())
        }
        fn store_entry(&mut self, value: super::EntryType) -> Result<(), Error> {
            self.entry.push(value);
            Ok(())
        }
        fn handle_proc_start<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
            fallback: &mut Option<PriopcdTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(PriopcdTypeDeserializerState::ProcStart(None));
                *self.state = PriopcdTypeDeserializerState::Entry(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_proc_start(data)?;
                    *self.state = PriopcdTypeDeserializerState::Entry(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                PriopcdTypeDeserializerState::ProcStart(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = PriopcdTypeDeserializerState::Entry(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = PriopcdTypeDeserializerState::ProcStart(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_entry<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::EntryType>,
            fallback: &mut Option<PriopcdTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.entry.len() < 1usize {
                    *self.state = PriopcdTypeDeserializerState::Entry(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(PriopcdTypeDeserializerState::Entry(None));
                    *self.state = PriopcdTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_entry(data)?;
                    *self.state = PriopcdTypeDeserializerState::Entry(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(PriopcdTypeDeserializerState::Entry(
                                Some(deserializer),
                            ));
                            if self.entry.len().saturating_add(1) < 1usize {
                                *self.state = PriopcdTypeDeserializerState::Entry(None);
                            } else {
                                *self.state = PriopcdTypeDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                PriopcdTypeDeserializerState::Entry(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::PriopcdType> for PriopcdTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PriopcdType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PriopcdType>
        where
            R: DeserializeReader,
        {
            use PriopcdTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::ProcStart(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_proc_start(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Entry(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_entry(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = PriopcdTypeDeserializerState::ProcStart(None);
                        event
                    }
                    (
                        S::ProcStart(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(&event, None, b"proc_start") {
                            let output = <i32 as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_proc_start(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Entry(None);
                            event
                        }
                    }
                    (S::Entry(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"entry") {
                            let output = < super :: EntryType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_entry(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PriopcdType, Error>
        where
            R: DeserializeReader,
        {
            let state =
                replace(&mut *self.state, PriopcdTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::PriopcdType {
                value: self.value,
                proc_start: self.proc_start,
                entry: self.entry,
            })
        }
    }
    #[derive(Debug)]
    pub struct RingnotesTypeDeserializer {
        ring_note: Vec<super::GennotesTypeGenNoteElementType>,
        state: Box<RingnotesTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum RingnotesTypeDeserializerState {
        Init__ , RingNote (Option << super :: GennotesTypeGenNoteElementType as WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl RingnotesTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                ring_note: Vec::new(),
                state: Box::new(RingnotesTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: RingnotesTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use RingnotesTypeDeserializerState as S;
            match state {
                S::RingNote(Some(deserializer)) => {
                    self.store_ring_note(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_ring_note(
            &mut self,
            value: super::GennotesTypeGenNoteElementType,
        ) -> Result<(), Error> {
            self.ring_note.push(value);
            Ok(())
        }
        fn handle_ring_note<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::GennotesTypeGenNoteElementType>,
            fallback: &mut Option<RingnotesTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.ring_note.len() < 1usize {
                    *self.state = RingnotesTypeDeserializerState::RingNote(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback
                        .get_or_insert(RingnotesTypeDeserializerState::RingNote(None));
                    *self.state = RingnotesTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_ring_note(data)?;
                    *self.state = RingnotesTypeDeserializerState::RingNote(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                RingnotesTypeDeserializerState::RingNote(Some(
                                    deserializer,
                                )),
                            );
                            if self.ring_note.len().saturating_add(1) < 1usize {
                                *self.state =
                                    RingnotesTypeDeserializerState::RingNote(None);
                            } else {
                                *self.state = RingnotesTypeDeserializerState::Done__;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = RingnotesTypeDeserializerState::RingNote(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::RingnotesType> for RingnotesTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RingnotesType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::RingnotesType>
        where
            R: DeserializeReader,
        {
            use RingnotesTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::RingNote(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_ring_note(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = RingnotesTypeDeserializerState::RingNote(None);
                        event
                    }
                    (S::RingNote(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"ring_note") {
                            let output = < super :: GennotesTypeGenNoteElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_ring_note(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::RingnotesType, Error>
        where
            R: DeserializeReader,
        {
            let state =
                replace(&mut *self.state, RingnotesTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::RingnotesType {
                ring_note: self.ring_note,
            })
        }
    }
    #[derive(Debug)]
    pub struct SkipmeTypeDeserializer {
        state: Box<SkipmeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SkipmeTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl SkipmeTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            Ok(Self {
                state: Box::new(SkipmeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SkipmeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::SkipmeType> for SkipmeTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SkipmeType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SkipmeType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: true,
                })
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SkipmeType, Error>
        where
            R: DeserializeReader,
        {
            let state =
                replace(&mut *self.state, SkipmeTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SkipmeType {})
        }
    }
    #[derive(Debug)]
    pub struct SyntaxTypeDeserializer {
        mod_: Option<String>,
        content: Vec<super::SyntaxTypeContent>,
        state: Box<SyntaxTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SyntaxTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::SyntaxTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SyntaxTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut mod_: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"mod" {
                    reader.read_attrib(&mut mod_, b"mod", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                mod_: mod_,
                content: Vec::new(),
                state: Box::new(SyntaxTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SyntaxTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let SyntaxTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: super::SyntaxTypeContent,
        ) -> Result<(), Error> {
            self.content.push(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SyntaxTypeContent>,
            fallback: &mut Option<SyntaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(SyntaxTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = SyntaxTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let can_have_more = self.content.len().saturating_add(1) < 73usize;
                    let ret = if can_have_more {
                        ElementHandlerOutput::from_event(event, allow_any)
                    } else {
                        ElementHandlerOutput::from_event_end(event, allow_any)
                    };
                    match (can_have_more, &ret) {
                        (true, ElementHandlerOutput::Continue { .. }) => {
                            fallback.get_or_insert(
                                SyntaxTypeDeserializerState::Content__(deserializer),
                            );
                            *self.state = SyntaxTypeDeserializerState::Next__;
                        }
                        (false, _) | (_, ElementHandlerOutput::Break { .. }) => {
                            *self.state =
                                SyntaxTypeDeserializerState::Content__(deserializer);
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SyntaxType> for SyntaxTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SyntaxType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SyntaxType>
        where
            R: DeserializeReader,
        {
            use SyntaxTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output = < super :: SyntaxTypeContent as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SyntaxType, Error>
        where
            R: DeserializeReader,
        {
            let state =
                replace(&mut *self.state, SyntaxTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::SyntaxType {
                mod_: self.mod_,
                content: self.content,
            })
        }
    }
    #[derive(Debug)]
    pub struct SyntaxTypeContentDeserializer {
        state: Box<SyntaxTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum SyntaxTypeContentDeserializerState {
        Init__,
        Mnem(
            Option<super::SyntaxTypeMnemElementType>,
            Option<<super::SyntaxTypeMnemElementType as WithDeserializer>::Deserializer>,
        ),
        Src(
            Option<super::OperandType>,
            Option<<super::OperandType as WithDeserializer>::Deserializer>,
        ),
        Dst(
            Option<super::OperandType>,
            Option<<super::OperandType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::SyntaxTypeContent),
        Unknown__,
    }
    impl SyntaxTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<SyntaxTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self.state = fallback
                    .take()
                    .unwrap_or(SyntaxTypeContentDeserializerState::Init__);
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if x.name().local_name().as_ref() == b"mnem" {
                let output = < super :: SyntaxTypeMnemElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                return self.handle_mnem(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if x.name().local_name().as_ref() == b"src" {
                let output =
                    <super::OperandType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_src(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            if x.name().local_name().as_ref() == b"dst" {
                let output =
                    <super::OperandType as WithDeserializer>::Deserializer::init(
                        reader, event,
                    )?;
                return self.handle_dst(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            *self.state = fallback
                .take()
                .unwrap_or(SyntaxTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: SyntaxTypeContentDeserializerState,
        ) -> Result<super::SyntaxTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use SyntaxTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Mnem(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_mnem(&mut values, value)?;
                    }
                    Ok(super::SyntaxTypeContent::Mnem(
                        values
                            .ok_or_else(|| ErrorKind::MissingElement("mnem".into()))?,
                    ))
                }
                S::Src(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_src(&mut values, value)?;
                    }
                    Ok(super::SyntaxTypeContent::Src(
                        values.ok_or_else(|| ErrorKind::MissingElement("src".into()))?,
                    ))
                }
                S::Dst(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_dst(&mut values, value)?;
                    }
                    Ok(super::SyntaxTypeContent::Dst(
                        values.ok_or_else(|| ErrorKind::MissingElement("dst".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
            }
        }
        fn store_mnem(
            values: &mut Option<super::SyntaxTypeMnemElementType>,
            value: super::SyntaxTypeMnemElementType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"mnem")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_src(
            values: &mut Option<super::OperandType>,
            value: super::OperandType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"src")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn store_dst(
            values: &mut Option<super::OperandType>,
            value: super::OperandType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"dst")))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_mnem<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::SyntaxTypeMnemElementType>,
            output: DeserializerOutput<'de, super::SyntaxTypeMnemElementType>,
            fallback: &mut Option<SyntaxTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SyntaxTypeContentDeserializerState::Init__,
                    Some(SyntaxTypeContentDeserializerState::Mnem(
                        _,
                        Some(deserializer),
                    )) => SyntaxTypeContentDeserializerState::Mnem(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SyntaxTypeContentDeserializerState::Mnem(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_mnem(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_mnem(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SyntaxTypeContentDeserializerState::Mnem(values, None),
                    )?;
                    *self.state = SyntaxTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = SyntaxTypeContentDeserializerState::Mnem(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_src<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OperandType>,
            output: DeserializerOutput<'de, super::OperandType>,
            fallback: &mut Option<SyntaxTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SyntaxTypeContentDeserializerState::Init__,
                    Some(SyntaxTypeContentDeserializerState::Src(
                        _,
                        Some(deserializer),
                    )) => SyntaxTypeContentDeserializerState::Src(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SyntaxTypeContentDeserializerState::Src(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_src(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_src(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SyntaxTypeContentDeserializerState::Src(values, None),
                    )?;
                    *self.state = SyntaxTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = SyntaxTypeContentDeserializerState::Src(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
        fn handle_dst<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::OperandType>,
            output: DeserializerOutput<'de, super::OperandType>,
            fallback: &mut Option<SyntaxTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => SyntaxTypeContentDeserializerState::Init__,
                    Some(SyntaxTypeContentDeserializerState::Dst(
                        _,
                        Some(deserializer),
                    )) => SyntaxTypeContentDeserializerState::Dst(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(SyntaxTypeContentDeserializerState::Dst(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_dst(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_dst(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        SyntaxTypeContentDeserializerState::Dst(values, None),
                    )?;
                    *self.state = SyntaxTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = SyntaxTypeContentDeserializerState::Dst(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SyntaxTypeContent> for SyntaxTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SyntaxTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(SyntaxTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(
                        &*x.state,
                        SyntaxTypeContentDeserializerState::Init__
                    ) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SyntaxTypeContent>
        where
            R: DeserializeReader,
        {
            use SyntaxTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Mnem(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_mnem(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Src(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_src(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Dst(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_dst(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        match self.find_suitable(reader, event, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Mnem(values, None), event) => {
                        let output = < super :: SyntaxTypeMnemElementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_mnem(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Src(values, None), event) => {
                        let output = < super :: OperandType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_src(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Dst(values, None), event) => {
                        let output = < super :: OperandType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_dst(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::SyntaxTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct X86ReferenceElementTypeDeserializer {
        version: Option<String>,
        one_byte: Option<super::OpcodeType>,
        two_byte: Option<super::OpcodeType>,
        gen_notes: Option<super::GennotesType>,
        ring_notes: Option<super::RingnotesType>,
        state: Box<X86ReferenceElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum X86ReferenceElementTypeDeserializerState {
        Init__,
        OneByte(Option<<super::OpcodeType as WithDeserializer>::Deserializer>),
        TwoByte(Option<<super::OpcodeType as WithDeserializer>::Deserializer>),
        GenNotes(Option<<super::GennotesType as WithDeserializer>::Deserializer>),
        RingNotes(Option<<super::RingnotesType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl X86ReferenceElementTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut version: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"version" {
                    reader.read_attrib(&mut version, b"version", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                version: version,
                one_byte: None,
                two_byte: None,
                gen_notes: None,
                ring_notes: None,
                state: Box::new(X86ReferenceElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: X86ReferenceElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use X86ReferenceElementTypeDeserializerState as S;
            match state {
                S::OneByte(Some(deserializer)) => {
                    self.store_one_byte(deserializer.finish(reader)?)?
                }
                S::TwoByte(Some(deserializer)) => {
                    self.store_two_byte(deserializer.finish(reader)?)?
                }
                S::GenNotes(Some(deserializer)) => {
                    self.store_gen_notes(deserializer.finish(reader)?)?
                }
                S::RingNotes(Some(deserializer)) => {
                    self.store_ring_notes(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_one_byte(&mut self, value: super::OpcodeType) -> Result<(), Error> {
            if self.one_byte.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"one-byte",
                )))?;
            }
            self.one_byte = Some(value);
            Ok(())
        }
        fn store_two_byte(&mut self, value: super::OpcodeType) -> Result<(), Error> {
            if self.two_byte.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"two-byte",
                )))?;
            }
            self.two_byte = Some(value);
            Ok(())
        }
        fn store_gen_notes(&mut self, value: super::GennotesType) -> Result<(), Error> {
            if self.gen_notes.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"gen_notes",
                )))?;
            }
            self.gen_notes = Some(value);
            Ok(())
        }
        fn store_ring_notes(
            &mut self,
            value: super::RingnotesType,
        ) -> Result<(), Error> {
            if self.ring_notes.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ring_notes",
                )))?;
            }
            self.ring_notes = Some(value);
            Ok(())
        }
        fn handle_one_byte<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::OpcodeType>,
            fallback: &mut Option<X86ReferenceElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.one_byte.is_some() {
                    fallback.get_or_insert(
                        X86ReferenceElementTypeDeserializerState::OneByte(None),
                    );
                    *self.state =
                        X86ReferenceElementTypeDeserializerState::TwoByte(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        X86ReferenceElementTypeDeserializerState::OneByte(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_one_byte(data)?;
                    *self.state =
                        X86ReferenceElementTypeDeserializerState::TwoByte(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                X86ReferenceElementTypeDeserializerState::OneByte(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                X86ReferenceElementTypeDeserializerState::TwoByte(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                X86ReferenceElementTypeDeserializerState::OneByte(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_two_byte<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::OpcodeType>,
            fallback: &mut Option<X86ReferenceElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.two_byte.is_some() {
                    fallback.get_or_insert(
                        X86ReferenceElementTypeDeserializerState::TwoByte(None),
                    );
                    *self.state =
                        X86ReferenceElementTypeDeserializerState::GenNotes(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        X86ReferenceElementTypeDeserializerState::TwoByte(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_two_byte(data)?;
                    *self.state =
                        X86ReferenceElementTypeDeserializerState::GenNotes(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                X86ReferenceElementTypeDeserializerState::TwoByte(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                X86ReferenceElementTypeDeserializerState::GenNotes(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                X86ReferenceElementTypeDeserializerState::TwoByte(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_gen_notes<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::GennotesType>,
            fallback: &mut Option<X86ReferenceElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.gen_notes.is_some() {
                    fallback.get_or_insert(
                        X86ReferenceElementTypeDeserializerState::GenNotes(None),
                    );
                    *self.state =
                        X86ReferenceElementTypeDeserializerState::RingNotes(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        X86ReferenceElementTypeDeserializerState::GenNotes(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_gen_notes(data)?;
                    *self.state =
                        X86ReferenceElementTypeDeserializerState::RingNotes(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                X86ReferenceElementTypeDeserializerState::GenNotes(
                                    Some(deserializer),
                                ),
                            );
                            *self.state =
                                X86ReferenceElementTypeDeserializerState::RingNotes(
                                    None,
                                );
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                X86ReferenceElementTypeDeserializerState::GenNotes(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_ring_notes<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::RingnotesType>,
            fallback: &mut Option<X86ReferenceElementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.ring_notes.is_some() {
                    fallback.get_or_insert(
                        X86ReferenceElementTypeDeserializerState::RingNotes(None),
                    );
                    *self.state = X86ReferenceElementTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        X86ReferenceElementTypeDeserializerState::RingNotes(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_ring_notes(data)?;
                    *self.state = X86ReferenceElementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                X86ReferenceElementTypeDeserializerState::RingNotes(
                                    Some(deserializer),
                                ),
                            );
                            *self.state =
                                X86ReferenceElementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                X86ReferenceElementTypeDeserializerState::RingNotes(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::X86ReferenceElementType>
        for X86ReferenceElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X86ReferenceElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::X86ReferenceElementType>
        where
            R: DeserializeReader,
        {
            use X86ReferenceElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::OneByte(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_one_byte(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TwoByte(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_two_byte(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::GenNotes(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_gen_notes(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::RingNotes(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_ring_notes(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            X86ReferenceElementTypeDeserializerState::OneByte(None);
                        event
                    }
                    (S::OneByte(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"one-byte") {
                            let output = < super :: OpcodeType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_one_byte(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::TwoByte(None);
                            event
                        }
                    }
                    (S::TwoByte(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"two-byte") {
                            let output = < super :: OpcodeType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_two_byte(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::GenNotes(None);
                            event
                        }
                    }
                    (S::GenNotes(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, None, b"gen_notes") {
                            let output = < super :: GennotesType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_gen_notes(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::RingNotes(None);
                            event
                        }
                    }
                    (
                        S::RingNotes(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(&event, None, b"ring_notes") {
                            let output = < super :: RingnotesType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_ring_notes(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(
            mut self,
            reader: &R,
        ) -> Result<super::X86ReferenceElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                X86ReferenceElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::X86ReferenceElementType {
                version: self.version,
                one_byte: self
                    .one_byte
                    .ok_or_else(|| ErrorKind::MissingElement("one-byte".into()))?,
                two_byte: self
                    .two_byte
                    .ok_or_else(|| ErrorKind::MissingElement("two-byte".into()))?,
                gen_notes: self
                    .gen_notes
                    .ok_or_else(|| ErrorKind::MissingElement("gen_notes".into()))?,
                ring_notes: self
                    .ring_notes
                    .ok_or_else(|| ErrorKind::MissingElement("ring_notes".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EntryTypeSecOpcdElementTypeDeserializer {
        escape: Option<super::YesNoType>,
        content: Option<String>,
        state: Box<EntryTypeSecOpcdElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EntryTypeSecOpcdElementTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EntryTypeSecOpcdElementTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut escape: Option<super::YesNoType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"escape" {
                    reader.read_attrib(&mut escape, b"escape", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                escape: escape,
                content: None,
                state: Box::new(EntryTypeSecOpcdElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: EntryTypeSecOpcdElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let EntryTypeSecOpcdElementTypeDeserializerState::Content__(
                deserializer,
            ) = state
            {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::EntryTypeSecOpcdElementType>
        where
            R: DeserializeReader,
        {
            use EntryTypeSecOpcdElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::EntryTypeSecOpcdElementType>
        for EntryTypeSecOpcdElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EntryTypeSecOpcdElementType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EntryTypeSecOpcdElementType>
        where
            R: DeserializeReader,
        {
            use EntryTypeSecOpcdElementTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(
            mut self,
            reader: &R,
        ) -> Result<super::EntryTypeSecOpcdElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                EntryTypeSecOpcdElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::EntryTypeSecOpcdElementType {
                escape: self.escape,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EntryTypeProcStartElementTypeDeserializer {
        post: Option<super::YesNoType>,
        lat_step: Option<super::YesNoType>,
        content: Option<i32>,
        state: Box<EntryTypeProcStartElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EntryTypeProcStartElementTypeDeserializerState {
        Init__,
        Content__(<i32 as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EntryTypeProcStartElementTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut post: Option<super::YesNoType> = None;
            let mut lat_step: Option<super::YesNoType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"post" {
                    reader.read_attrib(&mut post, b"post", &attrib.value)?;
                } else if attrib.key.local_name().as_ref() == b"lat_step" {
                    reader.read_attrib(&mut lat_step, b"lat_step", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                post: post,
                lat_step: lat_step,
                content: None,
                state: Box::new(EntryTypeProcStartElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: EntryTypeProcStartElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let EntryTypeProcStartElementTypeDeserializerState::Content__(
                deserializer,
            ) = state
            {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: i32) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, i32>,
        ) -> DeserializerResult<'de, super::EntryTypeProcStartElementType>
        where
            R: DeserializeReader,
        {
            use EntryTypeProcStartElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::EntryTypeProcStartElementType>
        for EntryTypeProcStartElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EntryTypeProcStartElementType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EntryTypeProcStartElementType>
        where
            R: DeserializeReader,
        {
            use EntryTypeProcStartElementTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(
            mut self,
            reader: &R,
        ) -> Result<super::EntryTypeProcStartElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                EntryTypeProcStartElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::EntryTypeProcStartElementType {
                post: self.post,
                lat_step: self.lat_step,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EntryTypeModifFElementTypeDeserializer {
        cond: Option<super::YesNoType>,
        content: Option<String>,
        state: Box<EntryTypeModifFElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EntryTypeModifFElementTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EntryTypeModifFElementTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut cond: Option<super::YesNoType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"cond" {
                    reader.read_attrib(&mut cond, b"cond", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                cond: cond,
                content: None,
                state: Box::new(EntryTypeModifFElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: EntryTypeModifFElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let EntryTypeModifFElementTypeDeserializerState::Content__(deserializer) =
                state
            {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::EntryTypeModifFElementType>
        where
            R: DeserializeReader,
        {
            use EntryTypeModifFElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::EntryTypeModifFElementType>
        for EntryTypeModifFElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EntryTypeModifFElementType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EntryTypeModifFElementType>
        where
            R: DeserializeReader,
        {
            use EntryTypeModifFElementTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(
            mut self,
            reader: &R,
        ) -> Result<super::EntryTypeModifFElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                EntryTypeModifFElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::EntryTypeModifFElementType {
                cond: self.cond,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct EntryTypeDefFElementTypeDeserializer {
        cond: Option<String>,
        content: Option<String>,
        state: Box<EntryTypeDefFElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum EntryTypeDefFElementTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl EntryTypeDefFElementTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut cond: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"cond" {
                    reader.read_attrib(&mut cond, b"cond", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                cond: cond,
                content: None,
                state: Box::new(EntryTypeDefFElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: EntryTypeDefFElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let EntryTypeDefFElementTypeDeserializerState::Content__(deserializer) =
                state
            {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::EntryTypeDefFElementType>
        where
            R: DeserializeReader,
        {
            use EntryTypeDefFElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::EntryTypeDefFElementType>
        for EntryTypeDefFElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EntryTypeDefFElementType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::EntryTypeDefFElementType>
        where
            R: DeserializeReader,
        {
            use EntryTypeDefFElementTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(
            mut self,
            reader: &R,
        ) -> Result<super::EntryTypeDefFElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                EntryTypeDefFElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::EntryTypeDefFElementType {
                cond: self.cond,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct GennotesTypeGenNoteElementTypeDeserializer {
        id: String,
        state: Box<GennotesTypeGenNoteElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum GennotesTypeGenNoteElementTypeDeserializerState {
        Init__,
        Unknown__,
    }
    impl GennotesTypeGenNoteElementTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut id: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"id" {
                    reader.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("id".into()))
                })?,
                state: Box::new(GennotesTypeGenNoteElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: GennotesTypeGenNoteElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            Ok(())
        }
    }
    impl<'de> Deserializer<'de, super::GennotesTypeGenNoteElementType>
        for GennotesTypeGenNoteElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GennotesTypeGenNoteElementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::GennotesTypeGenNoteElementType>
        where
            R: DeserializeReader,
        {
            if let Event::End(_) = &event {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Data(self.finish(reader)?),
                    event: DeserializerEvent::None,
                    allow_any: false,
                })
            } else {
                Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::Deserializer(self),
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                })
            }
        }
        fn finish<R>(
            mut self,
            reader: &R,
        ) -> Result<super::GennotesTypeGenNoteElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                GennotesTypeGenNoteElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::GennotesTypeGenNoteElementType { id: self.id })
        }
    }
    #[derive(Debug)]
    pub struct SyntaxTypeMnemElementTypeDeserializer {
        sug: Option<super::YesNoType>,
        content: Option<String>,
        state: Box<SyntaxTypeMnemElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SyntaxTypeMnemElementTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl SyntaxTypeMnemElementTypeDeserializer {
        fn from_bytes_start<R>(
            reader: &R,
            bytes_start: &BytesStart<'_>,
        ) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut sug: Option<super::YesNoType> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if attrib.key.local_name().as_ref() == b"sug" {
                    reader.read_attrib(&mut sug, b"sug", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                sug: sug,
                content: None,
                state: Box::new(SyntaxTypeMnemElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SyntaxTypeMnemElementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let SyntaxTypeMnemElementTypeDeserializerState::Content__(deserializer) =
                state
            {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::SyntaxTypeMnemElementType>
        where
            R: DeserializeReader,
        {
            use SyntaxTypeMnemElementTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::SyntaxTypeMnemElementType>
        for SyntaxTypeMnemElementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SyntaxTypeMnemElementType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SyntaxTypeMnemElementType>
        where
            R: DeserializeReader,
        {
            use SyntaxTypeMnemElementTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(
            mut self,
            reader: &R,
        ) -> Result<super::SyntaxTypeMnemElementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SyntaxTypeMnemElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SyntaxTypeMnemElementType {
                sug: self.sug,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
}
pub mod xs {
    use xsd_parser::quick_xml::{
        DeserializeBytes, DeserializeReader, Error, WithDeserializer,
    };
    #[derive(Debug, Default)]
    pub struct EntitiesType(pub Vec<String>);
    impl DeserializeBytes for EntitiesType {
        fn deserialize_bytes<R>(reader: &R, bytes: &[u8]) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| String::deserialize_bytes(reader, bytes))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    pub type EntityType = EntitiesType;
    pub type IdType = String;
    pub type IdrefType = String;
    pub type IdrefsType = EntitiesType;
    pub type NcNameType = String;
    pub type NmtokenType = String;
    pub type NmtokensType = EntitiesType;
    pub type NotationType = String;
    pub type NameType = String;
    pub type QNameType = String;
    #[derive(Debug)]
    pub struct AnyType;
    impl WithDeserializer for AnyType {
        type Deserializer = quick_xml_deserialize::AnyTypeDeserializer;
    }
    pub type AnyUriType = String;
    pub type Base64BinaryType = String;
    pub type BooleanType = bool;
    pub type ByteType = i8;
    pub type DateType = String;
    pub type DateTimeType = String;
    pub type DecimalType = f64;
    pub type DoubleType = f64;
    pub type DurationType = String;
    pub type FloatType = f32;
    pub type GDayType = String;
    pub type GMonthType = String;
    pub type GMonthDayType = String;
    pub type GYearType = String;
    pub type GYearMonthType = String;
    pub type HexBinaryType = String;
    pub type IntType = i32;
    pub type IntegerType = i32;
    pub type LanguageType = String;
    pub type LongType = i64;
    pub type NegativeIntegerType = isize;
    pub type NonNegativeIntegerType = usize;
    pub type NonPositiveIntegerType = isize;
    pub type NormalizedStringType = String;
    pub type PositiveIntegerType = usize;
    pub type ShortType = i16;
    pub type StringType = String;
    pub type TimeType = String;
    pub type TokenType = String;
    pub type UnsignedByteType = u8;
    pub type UnsignedIntType = u32;
    pub type UnsignedLongType = u64;
    pub type UnsignedShortType = u16;
    pub mod quick_xml_deserialize {
        use core::mem::replace;
        use xsd_parser::quick_xml::{
            BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
            DeserializerEvent, DeserializerOutput, DeserializerResult, Error, Event,
        };
        #[derive(Debug)]
        pub struct AnyTypeDeserializer {
            state: Box<AnyTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum AnyTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl AnyTypeDeserializer {
            fn from_bytes_start<R>(
                reader: &R,
                bytes_start: &BytesStart<'_>,
            ) -> Result<Self, Error>
            where
                R: DeserializeReader,
            {
                Ok(Self {
                    state: Box::new(AnyTypeDeserializerState::Init__),
                })
            }
            fn finish_state<R>(
                &mut self,
                reader: &R,
                state: AnyTypeDeserializerState,
            ) -> Result<(), Error>
            where
                R: DeserializeReader,
            {
                Ok(())
            }
        }
        impl<'de> Deserializer<'de, super::AnyType> for AnyTypeDeserializer {
            fn init<R>(
                reader: &R,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::AnyType>
            where
                R: DeserializeReader,
            {
                reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next<R>(
                mut self,
                reader: &R,
                event: Event<'de>,
            ) -> DeserializerResult<'de, super::AnyType>
            where
                R: DeserializeReader,
            {
                if let Event::End(_) = &event {
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(self.finish(reader)?),
                        event: DeserializerEvent::None,
                        allow_any: false,
                    })
                } else {
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event: DeserializerEvent::Break(event),
                        allow_any: true,
                    })
                }
            }
            fn finish<R>(mut self, reader: &R) -> Result<super::AnyType, Error>
            where
                R: DeserializeReader,
            {
                let state =
                    replace(&mut *self.state, AnyTypeDeserializerState::Unknown__);
                self.finish_state(reader, state)?;
                Ok(super::AnyType {})
            }
        }
    }
}
