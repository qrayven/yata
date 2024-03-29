#![allow(missing_docs)]

use crate::core::{ValueType, OHLCV};

pub mod example;

#[derive(Clone, Copy, Debug)]
struct HLC {
	high: ValueType,
	low: ValueType,
	close: ValueType,
}

impl HLC {
	fn from<T: OHLCV>(src: &T) -> Self {
		Self {
			high: src.high(),
			low: src.low(),
			close: src.close(),
		}
	}
}

impl OHLCV for HLC {
	fn open(&self) -> ValueType {
		ValueType::NAN
	}

	#[inline]
	fn high(&self) -> ValueType {
		self.high
	}

	#[inline]
	fn low(&self) -> ValueType {
		self.low
	}

	#[inline]
	fn close(&self) -> ValueType {
		self.close
	}

	fn volume(&self) -> ValueType {
		ValueType::NAN
	}
}

pub mod aroon;
pub use aroon::Aroon;

pub mod average_directional_index;
pub use average_directional_index::AverageDirectionalIndex;

pub mod awesome_oscillator;
pub use awesome_oscillator::AwesomeOscillator;

pub mod bollinger_bands;
pub use bollinger_bands::BollingerBands;

pub mod chaikin_money_flow;
pub use chaikin_money_flow::ChaikinMoneyFlow;

pub mod chaikin_oscillator;
pub use chaikin_oscillator::ChaikinOscillator;

pub mod chande_kroll_stop;
pub use chande_kroll_stop::ChandeKrollStop;

pub mod chande_momentum_oscillator;
pub use chande_momentum_oscillator::ChandeMomentumOscillator;

pub mod commodity_channel_index;
pub use commodity_channel_index::CommodityChannelIndex;

pub mod coppock_curve;
pub use coppock_curve::CoppockCurve;

pub mod detrended_price_oscillator;
pub use detrended_price_oscillator::DetrendedPriceOscillator;

pub mod donchian_channel;
pub use donchian_channel::DonchianChannel;

pub mod ease_of_movement;
pub use ease_of_movement::EaseOfMovement;

pub mod elders_force_index;
pub use elders_force_index::EldersForceIndex;

pub mod envelopes;
pub use envelopes::Envelopes;

pub mod fisher_transform;
pub use fisher_transform::FisherTransform;

pub mod hull_moving_average;
pub use hull_moving_average::HullMovingAverage;

pub mod ichimoku_cloud;
pub use ichimoku_cloud::IchimokuCloud;

pub mod kaufman;
pub use kaufman::{Kaufman, KAMA};

pub mod keltner_channel;
pub use keltner_channel::KeltnerChannel;

pub mod klinger_volume_oscillator;
pub use klinger_volume_oscillator::KlingerVolumeOscillator;

pub mod know_sure_thing;
pub use know_sure_thing::KnowSureThing;

pub mod macd;
pub use macd::{MovingAverageConvergenceDivergence, MACD};

pub mod momentum_index;
pub use momentum_index::MomentumIndex;

pub mod money_flow_index;
pub use money_flow_index::MoneyFlowIndex;

pub mod parabolic_sar;
pub use parabolic_sar::{ParabolicSAR, ParabolicStopAndReverse};

pub mod pivot_reversal_strategy;
pub use pivot_reversal_strategy::PivotReversalStrategy;

pub mod price_channel_strategy;
pub use price_channel_strategy::PriceChannelStrategy;

pub mod relative_strength_index;
pub use relative_strength_index::{RelativeStrengthIndex, RSI};

pub mod relative_vigor_index;
pub use relative_vigor_index::RelativeVigorIndex;

pub mod smi_ergodic_indicator;
pub use smi_ergodic_indicator::SMIErgodicIndicator;

pub mod stochastic_oscillator;
pub use stochastic_oscillator::StochasticOscillator;

pub mod trix;
pub use trix::Trix;

pub mod trend_strength_index;
pub use trend_strength_index::TrendStrengthIndex;

pub mod true_strength_index;
pub use true_strength_index::TrueStrengthIndex;

pub mod woodies_cci;
pub use woodies_cci::WoodiesCCI;
