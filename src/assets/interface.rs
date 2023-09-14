use crate::app::abort::AppResult;



/// #### 한국어 </br>
/// 모든 에셋 핸들의 내부 인터페이스 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is the internal interface for all asset handles. </br>
/// 
pub trait HandleInner : Sized {
    /// #### 한국어 </br>
    /// 에셋의 바이트 배열을 주어진 디코더로 디코딩하여 결과를 반환합니다. </br>
    /// 함수를 실행하는 도중 오류가 발생한 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Decode a byte array of assets with the given decoder and returns the result. </br>
    /// If an error occurs while executing the function, it returns `PanicMsg`. </br>
    /// 
    fn read<T, D: AssetDecoder<Output = T>>(&self) -> AppResult<D::Output>;

    /// #### 한국어 </br>
    /// 에셋의 바이트 배열을 주어진 디코더로 디코딩하여 결과를 반환합니다. </br>
    /// 선택적 유형인 경우 에셋 파일의 바이트 배열이 비어있을 때 
    /// 주어진 기본 값을 에셋 파일에 쓴 뒤 반환합니다. </br>
    /// 함수를 실행하는 도중 오류가 발생한 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Decode a byte array of assets with the given decoder and returns the result. </br>
    /// If this is an optional type, if the byte array in the asset file is empty,
    /// the given default value will be written to the asset and returned. </br>
    /// If an error occurs while executing the function, it returns `PanicMsg`. </br>
    /// 
    fn read_or_default<T: Default, D: AssetDecoder<Output = T>, E: AssetEncoder<Input = T>>(&mut self) -> AppResult<D::Output>;

    /// #### 한국어 </br>
    /// 에셋의 바이트 배열을 주어진 데이터로 채우고 에셋 파일에 덮어 씁니다. </br>
    /// 정적 유형의 에셋 파일의 경우 아무것도 수행하지 않습니다. </br>
    /// 함수를 실행하는 도중 오류가 발생한 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Fill the asset's byte array with the given data and overwrites its in the asset file. </br>
    /// For asset files of static type, it does nothing. </br>
    /// If an error occurs while executing the function, it returns `PanicMsg`. </br>
    /// 
    fn write<T, E: AssetEncoder<Input = T>>(&mut self, val: &E::Input) -> AppResult<()>;
}



/// #### 한국어 </br>
/// 에셋의 바이트 배열을 `Output`으로 디코딩 하는 인터페이스 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is an interface that decodes the byte array of assets into `Output`.
/// 
pub trait AssetDecoder {
    type Output;
    
    /// #### 한국어 </br>
    /// 주어진 에셋의 바이트 배열을 `Output`으로 디코딩 합니다. </br>
    /// 함수를 실행하는 도중 에러가 발생한 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Decodes the byte array of the given asset into `Output`. </br>
    /// If an error occurs while executing the function, `PanicMsg` is returned. </br>
    /// 
    fn decode(buf: &[u8]) -> AppResult<Self::Output>;
}



/// #### 한국어 </br>
/// `Input`을 에셋의 바이트 배열로 인코딩하는 인터페이스 입니다. </br>
/// 
/// #### English (Trnaslation)
/// This is an interface that encodes `Input` into a byte array of assets. </br>
/// 
pub trait AssetEncoder {
    type Input;

    /// #### 한국어 </br>
    /// 주어진 `Input`을 바이트 배열로 인코딩 합니다. </br>
    /// 함수를 실행하는 도중 에러가 발생한 경우 `PanicMsg`를 반환합니다. </br>
    /// 
    /// #### English (Translation) </br>
    /// Encodes the given `Input` into a byte array. </br>
    /// If an error occurs while executing the function, `PanicMsg` is returned. </br>
    /// 
    fn encode(val: &Self::Input) -> AppResult<Vec<u8>>;
}