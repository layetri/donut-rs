// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'commands.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models');

/// @nodoc
mixin _$PacketFromEngine {
  Object get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BigInt field0) position,
    required TResult Function(Float32List field0) buffer,
    required TResult Function(DevInfo field0) debugInfo,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt field0)? position,
    TResult? Function(Float32List field0)? buffer,
    TResult? Function(DevInfo field0)? debugInfo,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt field0)? position,
    TResult Function(Float32List field0)? buffer,
    TResult Function(DevInfo field0)? debugInfo,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PacketFromEngine_Position value) position,
    required TResult Function(PacketFromEngine_Buffer value) buffer,
    required TResult Function(PacketFromEngine_DebugInfo value) debugInfo,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PacketFromEngine_Position value)? position,
    TResult? Function(PacketFromEngine_Buffer value)? buffer,
    TResult? Function(PacketFromEngine_DebugInfo value)? debugInfo,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PacketFromEngine_Position value)? position,
    TResult Function(PacketFromEngine_Buffer value)? buffer,
    TResult Function(PacketFromEngine_DebugInfo value)? debugInfo,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $PacketFromEngineCopyWith<$Res> {
  factory $PacketFromEngineCopyWith(
          PacketFromEngine value, $Res Function(PacketFromEngine) then) =
      _$PacketFromEngineCopyWithImpl<$Res, PacketFromEngine>;
}

/// @nodoc
class _$PacketFromEngineCopyWithImpl<$Res, $Val extends PacketFromEngine>
    implements $PacketFromEngineCopyWith<$Res> {
  _$PacketFromEngineCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$PacketFromEngine_PositionImplCopyWith<$Res> {
  factory _$$PacketFromEngine_PositionImplCopyWith(
          _$PacketFromEngine_PositionImpl value,
          $Res Function(_$PacketFromEngine_PositionImpl) then) =
      __$$PacketFromEngine_PositionImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BigInt field0});
}

/// @nodoc
class __$$PacketFromEngine_PositionImplCopyWithImpl<$Res>
    extends _$PacketFromEngineCopyWithImpl<$Res,
        _$PacketFromEngine_PositionImpl>
    implements _$$PacketFromEngine_PositionImplCopyWith<$Res> {
  __$$PacketFromEngine_PositionImplCopyWithImpl(
      _$PacketFromEngine_PositionImpl _value,
      $Res Function(_$PacketFromEngine_PositionImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$PacketFromEngine_PositionImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BigInt,
    ));
  }
}

/// @nodoc

class _$PacketFromEngine_PositionImpl extends PacketFromEngine_Position {
  const _$PacketFromEngine_PositionImpl(this.field0) : super._();

  @override
  final BigInt field0;

  @override
  String toString() {
    return 'PacketFromEngine.position(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PacketFromEngine_PositionImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PacketFromEngine_PositionImplCopyWith<_$PacketFromEngine_PositionImpl>
      get copyWith => __$$PacketFromEngine_PositionImplCopyWithImpl<
          _$PacketFromEngine_PositionImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BigInt field0) position,
    required TResult Function(Float32List field0) buffer,
    required TResult Function(DevInfo field0) debugInfo,
  }) {
    return position(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt field0)? position,
    TResult? Function(Float32List field0)? buffer,
    TResult? Function(DevInfo field0)? debugInfo,
  }) {
    return position?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt field0)? position,
    TResult Function(Float32List field0)? buffer,
    TResult Function(DevInfo field0)? debugInfo,
    required TResult orElse(),
  }) {
    if (position != null) {
      return position(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PacketFromEngine_Position value) position,
    required TResult Function(PacketFromEngine_Buffer value) buffer,
    required TResult Function(PacketFromEngine_DebugInfo value) debugInfo,
  }) {
    return position(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PacketFromEngine_Position value)? position,
    TResult? Function(PacketFromEngine_Buffer value)? buffer,
    TResult? Function(PacketFromEngine_DebugInfo value)? debugInfo,
  }) {
    return position?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PacketFromEngine_Position value)? position,
    TResult Function(PacketFromEngine_Buffer value)? buffer,
    TResult Function(PacketFromEngine_DebugInfo value)? debugInfo,
    required TResult orElse(),
  }) {
    if (position != null) {
      return position(this);
    }
    return orElse();
  }
}

abstract class PacketFromEngine_Position extends PacketFromEngine {
  const factory PacketFromEngine_Position(final BigInt field0) =
      _$PacketFromEngine_PositionImpl;
  const PacketFromEngine_Position._() : super._();

  @override
  BigInt get field0;
  @JsonKey(ignore: true)
  _$$PacketFromEngine_PositionImplCopyWith<_$PacketFromEngine_PositionImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PacketFromEngine_BufferImplCopyWith<$Res> {
  factory _$$PacketFromEngine_BufferImplCopyWith(
          _$PacketFromEngine_BufferImpl value,
          $Res Function(_$PacketFromEngine_BufferImpl) then) =
      __$$PacketFromEngine_BufferImplCopyWithImpl<$Res>;
  @useResult
  $Res call({Float32List field0});
}

/// @nodoc
class __$$PacketFromEngine_BufferImplCopyWithImpl<$Res>
    extends _$PacketFromEngineCopyWithImpl<$Res, _$PacketFromEngine_BufferImpl>
    implements _$$PacketFromEngine_BufferImplCopyWith<$Res> {
  __$$PacketFromEngine_BufferImplCopyWithImpl(
      _$PacketFromEngine_BufferImpl _value,
      $Res Function(_$PacketFromEngine_BufferImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$PacketFromEngine_BufferImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as Float32List,
    ));
  }
}

/// @nodoc

class _$PacketFromEngine_BufferImpl extends PacketFromEngine_Buffer {
  const _$PacketFromEngine_BufferImpl(this.field0) : super._();

  @override
  final Float32List field0;

  @override
  String toString() {
    return 'PacketFromEngine.buffer(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PacketFromEngine_BufferImpl &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PacketFromEngine_BufferImplCopyWith<_$PacketFromEngine_BufferImpl>
      get copyWith => __$$PacketFromEngine_BufferImplCopyWithImpl<
          _$PacketFromEngine_BufferImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BigInt field0) position,
    required TResult Function(Float32List field0) buffer,
    required TResult Function(DevInfo field0) debugInfo,
  }) {
    return buffer(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt field0)? position,
    TResult? Function(Float32List field0)? buffer,
    TResult? Function(DevInfo field0)? debugInfo,
  }) {
    return buffer?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt field0)? position,
    TResult Function(Float32List field0)? buffer,
    TResult Function(DevInfo field0)? debugInfo,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PacketFromEngine_Position value) position,
    required TResult Function(PacketFromEngine_Buffer value) buffer,
    required TResult Function(PacketFromEngine_DebugInfo value) debugInfo,
  }) {
    return buffer(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PacketFromEngine_Position value)? position,
    TResult? Function(PacketFromEngine_Buffer value)? buffer,
    TResult? Function(PacketFromEngine_DebugInfo value)? debugInfo,
  }) {
    return buffer?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PacketFromEngine_Position value)? position,
    TResult Function(PacketFromEngine_Buffer value)? buffer,
    TResult Function(PacketFromEngine_DebugInfo value)? debugInfo,
    required TResult orElse(),
  }) {
    if (buffer != null) {
      return buffer(this);
    }
    return orElse();
  }
}

abstract class PacketFromEngine_Buffer extends PacketFromEngine {
  const factory PacketFromEngine_Buffer(final Float32List field0) =
      _$PacketFromEngine_BufferImpl;
  const PacketFromEngine_Buffer._() : super._();

  @override
  Float32List get field0;
  @JsonKey(ignore: true)
  _$$PacketFromEngine_BufferImplCopyWith<_$PacketFromEngine_BufferImpl>
      get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$PacketFromEngine_DebugInfoImplCopyWith<$Res> {
  factory _$$PacketFromEngine_DebugInfoImplCopyWith(
          _$PacketFromEngine_DebugInfoImpl value,
          $Res Function(_$PacketFromEngine_DebugInfoImpl) then) =
      __$$PacketFromEngine_DebugInfoImplCopyWithImpl<$Res>;
  @useResult
  $Res call({DevInfo field0});
}

/// @nodoc
class __$$PacketFromEngine_DebugInfoImplCopyWithImpl<$Res>
    extends _$PacketFromEngineCopyWithImpl<$Res,
        _$PacketFromEngine_DebugInfoImpl>
    implements _$$PacketFromEngine_DebugInfoImplCopyWith<$Res> {
  __$$PacketFromEngine_DebugInfoImplCopyWithImpl(
      _$PacketFromEngine_DebugInfoImpl _value,
      $Res Function(_$PacketFromEngine_DebugInfoImpl) _then)
      : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? field0 = null,
  }) {
    return _then(_$PacketFromEngine_DebugInfoImpl(
      null == field0
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as DevInfo,
    ));
  }
}

/// @nodoc

class _$PacketFromEngine_DebugInfoImpl extends PacketFromEngine_DebugInfo {
  const _$PacketFromEngine_DebugInfoImpl(this.field0) : super._();

  @override
  final DevInfo field0;

  @override
  String toString() {
    return 'PacketFromEngine.debugInfo(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$PacketFromEngine_DebugInfoImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$PacketFromEngine_DebugInfoImplCopyWith<_$PacketFromEngine_DebugInfoImpl>
      get copyWith => __$$PacketFromEngine_DebugInfoImplCopyWithImpl<
          _$PacketFromEngine_DebugInfoImpl>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BigInt field0) position,
    required TResult Function(Float32List field0) buffer,
    required TResult Function(DevInfo field0) debugInfo,
  }) {
    return debugInfo(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt field0)? position,
    TResult? Function(Float32List field0)? buffer,
    TResult? Function(DevInfo field0)? debugInfo,
  }) {
    return debugInfo?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt field0)? position,
    TResult Function(Float32List field0)? buffer,
    TResult Function(DevInfo field0)? debugInfo,
    required TResult orElse(),
  }) {
    if (debugInfo != null) {
      return debugInfo(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(PacketFromEngine_Position value) position,
    required TResult Function(PacketFromEngine_Buffer value) buffer,
    required TResult Function(PacketFromEngine_DebugInfo value) debugInfo,
  }) {
    return debugInfo(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(PacketFromEngine_Position value)? position,
    TResult? Function(PacketFromEngine_Buffer value)? buffer,
    TResult? Function(PacketFromEngine_DebugInfo value)? debugInfo,
  }) {
    return debugInfo?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(PacketFromEngine_Position value)? position,
    TResult Function(PacketFromEngine_Buffer value)? buffer,
    TResult Function(PacketFromEngine_DebugInfo value)? debugInfo,
    required TResult orElse(),
  }) {
    if (debugInfo != null) {
      return debugInfo(this);
    }
    return orElse();
  }
}

abstract class PacketFromEngine_DebugInfo extends PacketFromEngine {
  const factory PacketFromEngine_DebugInfo(final DevInfo field0) =
      _$PacketFromEngine_DebugInfoImpl;
  const PacketFromEngine_DebugInfo._() : super._();

  @override
  DevInfo get field0;
  @JsonKey(ignore: true)
  _$$PacketFromEngine_DebugInfoImplCopyWith<_$PacketFromEngine_DebugInfoImpl>
      get copyWith => throw _privateConstructorUsedError;
}
