#[macro_export]
macro_rules! cfg_feature {
    ($feature:literal, $($item:item)*) => {
        $(
            #[cfg(feature = $feature)]
            #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
            $item
        )*
    }
}

macro_rules! cfg_client {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature = "jsonrpsee-http-client", feature = "jsonrpsee-ws-client", feature = "client", feature = "async-client"))]
            $item
        )*
    }
}

macro_rules! cfg_http_client {
	($($item:item)*) => {
		$crate::cfg_feature!("jsonrpsee-http-client", $($item)*);
	};
}

macro_rules! cfg_ws_client {
	($($item:item)*) => {
		$crate::cfg_feature!("jsonrpsee-ws-client", $($item)*);
	};
}

macro_rules! cfg_wasm_client {
	($($item:item)*) => {
		$crate::cfg_feature!("jsonrpsee-wasm-client", $($item)*);
	};
}

macro_rules! cfg_async_client {
  	($($item:item)*) => {
		$crate::cfg_feature!("async-client", $($item)*);
	};
}

macro_rules! cfg_client_transport {
    ($($item:item)*) => {
		$crate::cfg_feature!("jsonrpsee-client-transport", $($item)*);
	};
}

macro_rules! cfg_server {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature = "jsonrpsee-http-server", feature = "jsonrpsee-ws-server"))]
            $item
        )*
    }
}

macro_rules! cfg_http_server {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "jsonrpsee-http-server")]
            #[cfg_attr(docsrs, doc(cfg(feature = "jsonrpsee-http-server")))]
            $item
        )*
    }
}

macro_rules! cfg_ws_server {
     ($($item:item)*) => {
		$crate::cfg_feature!("jsonrpsee-ws-server", $($item)*);
	};
}

macro_rules! cfg_proc_macros {
    ($($item:item)*) => {
		$crate::cfg_feature!("jsonrpsee-proc-macros", $($item)*);
	};
}

macro_rules! cfg_types {
  ($($item:item)*) => {
		$crate::cfg_feature!("jsonrpsee-types", $($item)*);
    };
}

macro_rules! cfg_client_or_server {
    ($($item:item)*) => {
        $(
            #[cfg(any(feature = "jsonrpsee-http-client", feature = "jsonrpsee-ws-client", feature = "client", feature = "async-client", feature = "jsonrpsee-ws-server", feature = "jsonrpsee-http-client"))]
            $item
        )*
    }
}