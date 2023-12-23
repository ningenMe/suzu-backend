/**
 * @fileoverview gRPC-Web generated client stub for suzu
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck


import * as grpcWeb from 'grpc-web';

import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as api_proto_suzu_pb from '../../api/proto/suzu_pb';


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
    '/suzu.BlogService/GetBlog',
    grpcWeb.MethodType.UNARY,
    google_protobuf_empty_pb.Empty,
    api_proto_suzu_pb.GetBlogResponse,
    (request: google_protobuf_empty_pb.Empty) => {
      return request.serializeBinary();
    },
    api_proto_suzu_pb.GetBlogResponse.deserializeBinary
  );

  getBlog(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | null): Promise<api_proto_suzu_pb.GetBlogResponse>;

  getBlog(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: api_proto_suzu_pb.GetBlogResponse) => void): grpcWeb.ClientReadableStream<api_proto_suzu_pb.GetBlogResponse>;

  getBlog(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: api_proto_suzu_pb.GetBlogResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/suzu.BlogService/GetBlog',
        request,
        metadata || {},
        this.methodDescriptorGetBlog,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/suzu.BlogService/GetBlog',
    request,
    metadata || {},
    this.methodDescriptorGetBlog);
  }

}

