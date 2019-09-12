/*
 * Copyright 2018 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Generated by the Scala Plugin for the Protocol Buffer Compiler.
// Do not edit!
//
// Protofile syntax: PROTO3

package proto3.tendermint

/** @param proof
 *   it's actually a struct – SimpleProof from simple_proof.go, but we don't care just yet
 */
@SerialVersionUID(0L)
final case class BlockPart(
  index: _root_.scala.Long = 0L,
  bytes: _root_.com.google.protobuf.ByteString = _root_.com.google.protobuf.ByteString.EMPTY,
  proof: _root_.com.google.protobuf.ByteString = _root_.com.google.protobuf.ByteString.EMPTY
) extends scalapb.GeneratedMessage with scalapb.Message[BlockPart] with scalapb.lenses.Updatable[BlockPart] {

  @transient
  private[this] var __serializedSizeCachedValue: _root_.scala.Int = 0
  private[this] def __computeSerializedValue(): _root_.scala.Int = {
    var __size = 0

    {
      val __value = index
      if (__value != 0L) {
        __size += _root_.com.google.protobuf.CodedOutputStream.computeInt64Size(1, __value)
      }
    };

    {
      val __value = bytes
      if (__value != _root_.com.google.protobuf.ByteString.EMPTY) {
        __size += _root_.com.google.protobuf.CodedOutputStream.computeBytesSize(2, __value)
      }
    };

    {
      val __value = proof
      if (__value != _root_.com.google.protobuf.ByteString.EMPTY) {
        __size += _root_.com.google.protobuf.CodedOutputStream.computeBytesSize(3, __value)
      }
    };
    __size
  }
  final override def serializedSize: _root_.scala.Int = {
    var read = __serializedSizeCachedValue
    if (read == 0) {
      read = __computeSerializedValue()
      __serializedSizeCachedValue = read
    }
    read
  }

  def writeTo(`_output__`: _root_.com.google.protobuf.CodedOutputStream): _root_.scala.Unit = {
    {
      val __v = index
      if (__v != 0L) {
        _output__.writeInt64(1, __v)
      }
    };
    {
      val __v = bytes
      if (__v != _root_.com.google.protobuf.ByteString.EMPTY) {
        _output__.writeBytes(2, __v)
      }
    };
    {
      val __v = proof
      if (__v != _root_.com.google.protobuf.ByteString.EMPTY) {
        _output__.writeBytes(3, __v)
      }
    };
  }

  def mergeFrom(`_input__`: _root_.com.google.protobuf.CodedInputStream): proto3.tendermint.BlockPart = {
    var __index = this.index
    var __bytes = this.bytes
    var __proof = this.proof
    var _done__ = false
    while (!_done__) {
      val _tag__ = _input__.readTag()
      _tag__ match {
        case 0 => _done__ = true
        case 8 =>
          __index = _input__.readInt64()
        case 18 =>
          __bytes = _input__.readBytes()
        case 26 =>
          __proof = _input__.readBytes()
        case tag => _input__.skipField(tag)
      }
    }
    proto3.tendermint.BlockPart(
      index = __index,
      bytes = __bytes,
      proof = __proof
    )
  }
  def withIndex(__v: _root_.scala.Long): BlockPart = copy(index = __v)
  def withBytes(__v: _root_.com.google.protobuf.ByteString): BlockPart = copy(bytes = __v)
  def withProof(__v: _root_.com.google.protobuf.ByteString): BlockPart = copy(proof = __v)

  def getFieldByNumber(__fieldNumber: _root_.scala.Int): _root_.scala.Any = {
    (__fieldNumber: @ _root_.scala.unchecked) match {
      case 1 => {
        val __t = index
        if (__t != 0L) __t else null
      }
      case 2 => {
        val __t = bytes
        if (__t != _root_.com.google.protobuf.ByteString.EMPTY) __t else null
      }
      case 3 => {
        val __t = proof
        if (__t != _root_.com.google.protobuf.ByteString.EMPTY) __t else null
      }
    }
  }

  def getField(__field: _root_.scalapb.descriptors.FieldDescriptor): _root_.scalapb.descriptors.PValue = {
    _root_.scala.Predef.require(__field.containingMessage eq companion.scalaDescriptor)
    (__field.number: @ _root_.scala.unchecked) match {
      case 1 => _root_.scalapb.descriptors.PLong(index)
      case 2 => _root_.scalapb.descriptors.PByteString(bytes)
      case 3 => _root_.scalapb.descriptors.PByteString(proof)
    }
  }
  def toProtoString: _root_.scala.Predef.String = _root_.scalapb.TextFormat.printToUnicodeString(this)
  def companion = proto3.tendermint.BlockPart
}

object BlockPart extends scalapb.GeneratedMessageCompanion[proto3.tendermint.BlockPart] {
  implicit def messageCompanion: scalapb.GeneratedMessageCompanion[proto3.tendermint.BlockPart] = this

  def fromFieldsMap(
    __fieldsMap: scala.collection.immutable.Map[
      _root_.com.google.protobuf.Descriptors.FieldDescriptor,
      _root_.scala.Any
    ]
  ): proto3.tendermint.BlockPart = {
    _root_.scala.Predef.require(
      __fieldsMap.keys.forall(_.getContainingType() == javaDescriptor),
      "FieldDescriptor does not match message type."
    )
    val __fields = javaDescriptor.getFields
    proto3.tendermint.BlockPart(
      __fieldsMap.getOrElse(__fields.get(0), 0L).asInstanceOf[_root_.scala.Long],
      __fieldsMap
        .getOrElse(__fields.get(1), _root_.com.google.protobuf.ByteString.EMPTY)
        .asInstanceOf[_root_.com.google.protobuf.ByteString],
      __fieldsMap
        .getOrElse(__fields.get(2), _root_.com.google.protobuf.ByteString.EMPTY)
        .asInstanceOf[_root_.com.google.protobuf.ByteString]
    )
  }
  implicit def messageReads: _root_.scalapb.descriptors.Reads[proto3.tendermint.BlockPart] =
    _root_.scalapb.descriptors.Reads {
      case _root_.scalapb.descriptors.PMessage(__fieldsMap) =>
        _root_.scala.Predef.require(
          __fieldsMap.keys.forall(_.containingMessage == scalaDescriptor),
          "FieldDescriptor does not match message type."
        )
        proto3.tendermint.BlockPart(
          __fieldsMap.get(scalaDescriptor.findFieldByNumber(1).get).map(_.as[_root_.scala.Long]).getOrElse(0L),
          __fieldsMap
            .get(scalaDescriptor.findFieldByNumber(2).get)
            .map(_.as[_root_.com.google.protobuf.ByteString])
            .getOrElse(_root_.com.google.protobuf.ByteString.EMPTY),
          __fieldsMap
            .get(scalaDescriptor.findFieldByNumber(3).get)
            .map(_.as[_root_.com.google.protobuf.ByteString])
            .getOrElse(_root_.com.google.protobuf.ByteString.EMPTY)
        )
      case _ => throw new RuntimeException("Expected PMessage")
    }

  def javaDescriptor: _root_.com.google.protobuf.Descriptors.Descriptor =
    TendermintProto.javaDescriptor.getMessageTypes.get(12)
  def scalaDescriptor: _root_.scalapb.descriptors.Descriptor = TendermintProto.scalaDescriptor.messages(12)

  def messageCompanionForFieldNumber(__number: _root_.scala.Int): _root_.scalapb.GeneratedMessageCompanion[_] =
    throw new MatchError(__number)
  lazy val nestedMessagesCompanions: Seq[_root_.scalapb.GeneratedMessageCompanion[_]] = Seq.empty

  def enumCompanionForFieldNumber(__fieldNumber: _root_.scala.Int): _root_.scalapb.GeneratedEnumCompanion[_] =
    throw new MatchError(__fieldNumber)
  lazy val defaultInstance = proto3.tendermint.BlockPart(
    )
  implicit class BlockPartLens[UpperPB](_l: _root_.scalapb.lenses.Lens[UpperPB, proto3.tendermint.BlockPart])
      extends _root_.scalapb.lenses.ObjectLens[UpperPB, proto3.tendermint.BlockPart](_l) {
    def index: _root_.scalapb.lenses.Lens[UpperPB, _root_.scala.Long] = field(_.index)((c_, f_) => c_.copy(index = f_))

    def bytes: _root_.scalapb.lenses.Lens[UpperPB, _root_.com.google.protobuf.ByteString] =
      field(_.bytes)((c_, f_) => c_.copy(bytes = f_))

    def proof: _root_.scalapb.lenses.Lens[UpperPB, _root_.com.google.protobuf.ByteString] =
      field(_.proof)((c_, f_) => c_.copy(proof = f_))
  }
  final val INDEX_FIELD_NUMBER = 1
  final val BYTES_FIELD_NUMBER = 2
  final val PROOF_FIELD_NUMBER = 3
}