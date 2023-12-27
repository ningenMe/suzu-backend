/**
 * @fileoverview gRPC-Web generated client stub for suzu.v1
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck


import * as grpcWeb from 'grpc-web';

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as api_proto_suzu_v1_suzu_pb from '../../../../api/proto/suzu/v1/suzu_pb';


export class BlogServiceClient {
  client_: grpcWeb.AbstractClientBase;
  hostname_: string;
  credentials_: null | { [index: string]: string; };
  options_: null | { [index: string]: any; };

  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; }) {
    if (!options) options = {};
    if (!credentials) credentials = {};
    options['format'] = 'text';

    this.client_ = new grpcWeb.GrpcWebClientBase(options);
    this.hostname_ = hostname;
    this.credentials_ = credentials;
    this.options_ = options;
  }

  methodDescriptorGetBlog = new grpcWeb.MethodDescriptor(
    '/suzu.v1.BlogService/GetBlog',
    grpcWeb.MethodType.UNARY,
    google_protobuf_empty_pb.Empty,
    api_proto_suzu_v1_suzu_pb.GetBlogResponse,
    (request: google_protobuf_empty_pb.Empty) => {
      return request.serializeBinary();
    },
    api_proto_suzu_v1_suzu_pb.GetBlogResponse.deserializeBinary
  );

  getBlog(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | null): Promise<api_proto_suzu_v1_suzu_pb.GetBlogResponse>;

  getBlog(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: api_proto_suzu_v1_suzu_pb.GetBlogResponse) => void): grpcWeb.ClientReadableStream<api_proto_suzu_v1_suzu_pb.GetBlogResponse>;

  getBlog(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: api_proto_suzu_v1_suzu_pb.GetBlogResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/suzu.v1.BlogService/GetBlog',
        request,
        metadata || {},
        this.methodDescriptorGetBlog,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/suzu.v1.BlogService/GetBlog',
    request,
    metadata || {},
    this.methodDescriptorGetBlog);
  }

  methodDescriptorGetHealth = new grpcWeb.MethodDescriptor(
    '/suzu.v1.BlogService/GetHealth',
    grpcWeb.MethodType.UNARY,
    google_protobuf_empty_pb.Empty,
    api_proto_suzu_v1_suzu_pb.GetHealthResponse,
    (request: google_protobuf_empty_pb.Empty) => {
      return request.serializeBinary();
    },
    api_proto_suzu_v1_suzu_pb.GetHealthResponse.deserializeBinary
  );

  getHealth(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | null): Promise<api_proto_suzu_v1_suzu_pb.GetHealthResponse>;

  getHealth(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: api_proto_suzu_v1_suzu_pb.GetHealthResponse) => void): grpcWeb.ClientReadableStream<api_proto_suzu_v1_suzu_pb.GetHealthResponse>;

  getHealth(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: api_proto_suzu_v1_suzu_pb.GetHealthResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/suzu.v1.BlogService/GetHealth',
        request,
        metadata || {},
        this.methodDescriptorGetHealth,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/suzu.v1.BlogService/GetHealth',
    request,
    metadata || {},
    this.methodDescriptorGetHealth);
  }

}

